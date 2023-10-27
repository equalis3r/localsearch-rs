use crate::errors::LocalSearchError;
use crate::problem::{CostFunction, Neighborhood, Problem};
use crate::solver::Solver;
use crate::termination::{Reason, Status};
use crate::{Iteration, State};
use rand::Rng;
use rayon::prelude::*;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct TabuSearch<R, P> {
    num_neighbors: Option<u32>,
    tabu_list: VecDeque<P>,
    stall_iter_accepted: u32,
    stall_iter_accepted_limit: u32,
    stall_iter_best: u32,
    stall_iter_best_limit: u32,
    init_temp: f64,
    rng: R,
}

impl<R: Rng, P> TabuSearch<R, P> {
    pub fn new(num_neighbors: Option<u32>, capacity: usize, rng: R) -> Self {
        Self {
            num_neighbors,
            tabu_list: VecDeque::with_capacity(capacity),
            stall_iter_accepted: 0,
            stall_iter_accepted_limit: u32::MAX,
            stall_iter_best: 0,
            stall_iter_best_limit: u32::MAX,
            init_temp: 100.0f64,
            rng,
        }
    }

    #[must_use]
    pub fn with_stall_accepted(mut self, iter: u32) -> Self {
        self.stall_iter_accepted_limit = iter;
        self
    }

    #[must_use]
    pub fn with_stall_best(mut self, iter: u32) -> Self {
        self.stall_iter_best_limit = iter;
        self
    }

    #[must_use]
    pub fn with_init_temp(mut self, init_temp: f64) -> Self {
        self.init_temp = init_temp;
        self
    }

    fn update_stall_iter(&mut self, accepted: bool, new_best: bool) {
        self.stall_iter_accepted = if accepted {
            0
        } else {
            self.stall_iter_accepted + 1
        };

        self.stall_iter_best = if new_best {
            0
        } else {
            self.stall_iter_best + 1
        };
    }
}

impl<O, P, R, N> Solver<O, Iteration<P>> for TabuSearch<R, P>
where
    O: CostFunction<Param = P> + Neighborhood<Param = P, Neighbor = N> + Send + Sync,
    P: Clone + PartialEq + Send + Sync,
    R: Rng,
    N: Sized + Send + Sync,
{
    const NAME: &'static str = "TabuSearch";

    fn init(
        &mut self,
        problem: &mut Problem<O>,
        mut state: Iteration<P>,
    ) -> Result<Iteration<P>, LocalSearchError> {
        let param = state.take_param().unwrap();

        let cost = state.get_cost();
        let cost = if cost.is_infinite() {
            problem.cost(&param)?
        } else {
            cost
        };

        Ok(state.param(param).cost(cost))
    }

    fn next_iter(
        &mut self,
        problem: &mut Problem<O>,
        mut state: Iteration<P>,
    ) -> Result<Iteration<P>, LocalSearchError> {
        let prev_param = state.take_param().unwrap();
        let prev_cost = state.get_cost();

        let neighbors = problem
            .get_neighbors(&mut self.rng, &prev_param, self.num_neighbors)
            .unwrap();
        let res = neighbors.into_par_iter().map(|neighbor| {
            problem.make_move(&prev_param, neighbor).map_or_else(
                |_| Err(LocalSearchError::FailGenCandidateState),
                |param| {
                    let new_cost = problem.cost(&param).unwrap();
                    Ok((param, new_cost))
                },
            )
        });

        let (new_state, new_cost) = res
            .filter_map(core::result::Result::ok)
            .filter(|(cand_state, _)| !self.tabu_list.contains(cand_state))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map_or_else(|| (prev_param.clone(), prev_cost), |val| val);

        if self.tabu_list.len() == self.tabu_list.capacity() {
            self.tabu_list.pop_front();
        }
        self.tabu_list.push_back(new_state.clone());

        let delta = new_cost - prev_cost;
        let accepted = (delta.is_sign_negative() & (delta.abs() > f64::EPSILON))
            || (1.0 / (1.0 + f64::from(state.get_iter() + 1).powf(delta / self.init_temp))
                > self.rng.gen());

        let new_best_found = new_cost < state.best_cost;
        self.update_stall_iter(accepted, new_best_found);

        Ok(if accepted {
            state.param(new_state).cost(new_cost)
        } else {
            state.param(prev_param).cost(prev_cost)
        })
    }

    fn terminate(&mut self) -> Status {
        if self.stall_iter_best_limit < self.stall_iter_best {
            return Status::Terminated(Reason::MaxStallBestReached);
        }
        Status::NotTerminated
    }
}
