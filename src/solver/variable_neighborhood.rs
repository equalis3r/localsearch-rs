use crate::errors::LocalSearchError;
use crate::problem::{CostFunction, Neighborhood};
use crate::solver::Solver;
use crate::termination::{Reason, Status};
use crate::{IterState, State};
use rand::Rng;
use rayon::prelude::*;

#[derive(Clone)]
pub struct VariableNeighborhood<R, N> {
    num_neighbors: Option<u32>,
    cur_neighbors: Option<Vec<N>>,
    init_temp: f64,
    stall_iter_best: u32,
    stall_iter_best_limit: u32,
    rng: R,
}

impl<R: Rng, N> VariableNeighborhood<R, N> {
    pub fn new(num_neighbors: Option<u32>, rng: R) -> Self {
        Self {
            num_neighbors,
            cur_neighbors: None,
            init_temp: 100.0,
            stall_iter_best: 0,
            stall_iter_best_limit: u32::MAX,
            rng,
        }
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

    fn update_stall_iter(&mut self, new_best: bool) {
        self.stall_iter_best = if new_best {
            0
        } else {
            self.stall_iter_best + 1
        };
    }
}

impl<O, P, R, N> Solver<O, IterState<P>> for VariableNeighborhood<R, N>
where
    O: CostFunction<Param = P> + Neighborhood<Param = P, Neighbor = N> + Send + Sync,
    P: Clone + Send + Sync,
    R: Rng,
    N: Clone + Send + Sync,
{
    const NAME: &'static str = "VariableNeighborhood";

    fn init(
        &mut self,
        problem: &mut O,
        mut state: IterState<P>,
    ) -> Result<IterState<P>, LocalSearchError> {
        let param = state.take_param().unwrap();
        let cost = problem.cost(&param)?;
        Ok(state.param(param).cost(cost))
    }

    fn next_iter(
        &mut self,
        problem: &mut O,
        mut state: IterState<P>,
    ) -> Result<IterState<P>, LocalSearchError> {
        let prev_param = state.take_param().unwrap();
        let prev_cost = state.get_cost();

        let mut neighbors = match self.cur_neighbors.take() {
            Some(n) => n,
            None => problem.get_neighbor_moves(&mut self.rng, &prev_param)?,
        };

        let eval_neighbors = match self.num_neighbors {
            Some(val) => {
                if val as usize <= neighbors.len() {
                    neighbors.drain(..val as usize)
                } else {
                    neighbors.drain(..)
                }
            }
            None => neighbors.drain(..),
        };

        let res = eval_neighbors.par_bridge().map(|neighbor| {
            problem
                .get_neighbor_delta(&prev_param, &neighbor)
                .map_or_else(
                    |_| Err(LocalSearchError::FailGenCandidateState),
                    |new_cost| Ok((neighbor, new_cost)),
                )
        });

        let (neighbor, delta) = res
            .filter_map(core::result::Result::ok)
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map_or_else(|| (None, 0.0), |(neighbor, cost)| (Some(neighbor), cost));

        let new_state = neighbor.map_or_else(
            || prev_param.clone(),
            |n| problem.make_move(&prev_param, &n).unwrap(),
        );

        let accepted = (delta.is_sign_negative() && (delta.abs() > f64::EPSILON))
            || (1.0 / (1.0 + f64::from(state.get_iter() + 1).powf(delta / self.init_temp))
                > self.rng.gen());

        let new_cost = prev_cost + delta;
        let new_best_found = new_cost < state.best_cost;
        self.update_stall_iter(new_best_found);

        if accepted {
            self.cur_neighbors = None;
            Ok(state.param(new_state).cost(new_cost))
        } else {
            if neighbors.is_empty() {
                self.cur_neighbors = None;
            } else {
                self.cur_neighbors = Some(neighbors);
            }
            Ok(state.param(prev_param).cost(prev_cost))
        }
    }

    fn terminate(&mut self) -> Status {
        if self.stall_iter_best_limit < self.stall_iter_best {
            return Status::Terminated(Reason::MaxStallBestReached);
        }
        Status::NotTerminated
    }
}
