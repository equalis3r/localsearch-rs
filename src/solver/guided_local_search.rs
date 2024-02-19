use crate::errors::LocalSearchError;
use crate::problem::{AugmentedNeighborhood, CostFunction, Neighborhood, Penalty};
use crate::solver::Solver;
use crate::termination::{Reason, Status};
use crate::IterState;
use rand::Rng;
use rayon::prelude::*;
use std::hash::Hash;

pub struct GuidedLocalSearch<R, N, F: Hash + Eq> {
    num_neighbors: Option<u32>,
    cur_neighbors: Option<Vec<N>>,
    penalty: Penalty<F>,
    stall_iter_best: u32,
    stall_iter_best_limit: u32,
    rng: R,
}

impl<R, N, F> GuidedLocalSearch<R, N, F>
where
    R: Rng,
    F: Hash + Eq,
{
    pub fn new(num_neighbors: Option<u32>, alpha: f64, rng: R) -> Self {
        Self {
            num_neighbors,
            cur_neighbors: None,
            penalty: Penalty::new(alpha),
            stall_iter_best: 0,
            stall_iter_best_limit: u32::MAX,
            rng,
        }
    }

    #[must_use]
    pub fn replace_penalty(mut self, penalty: Penalty<F>) -> Self {
        self.penalty = penalty;
        self
    }

    pub fn get_penalty(&self) -> &Penalty<F> {
        &self.penalty
    }

    pub fn calibrate_penalty(&mut self, ratio: f64) {
        self.penalty.calibrate(ratio);
    }

    #[must_use]
    pub fn with_stall_best(mut self, iter: u32) -> Self {
        self.stall_iter_best = 0;
        self.stall_iter_best_limit = iter;
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

impl<O, P, R, N, F> Solver<O, IterState<P>> for GuidedLocalSearch<R, N, F>
where
    O: CostFunction<Param = P>
        + Neighborhood<Param = P, Neighbor = N>
        + AugmentedNeighborhood<F, Param = P, Neighbor = N, Penalty = Penalty<F>>
        + Send
        + Sync,
    P: Clone + Send + Sync,
    R: Rng,
    N: Clone + Send + Sync,
    F: Hash + Eq + Send + Sync,
{
    const NAME: &'static str = "GuidedLocalSearch";

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
                .get_neighbor_augmented_delta(&prev_param, &neighbor, &self.penalty)
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

        let mut accepted = delta.is_sign_negative() && (delta.abs() > f64::EPSILON);

        let original_cost = problem.cost(&new_state)?;
        let new_best_found = original_cost < state.best_cost;
        self.update_stall_iter(new_best_found);

        if neighbors.is_empty() {
            self.cur_neighbors = None;
        } else {
            self.cur_neighbors = Some(neighbors);
        }

        if self.cur_neighbors.is_none() && !accepted {
            problem.update_penalty(&prev_param, &mut self.penalty)?;
            self.penalty.lambda = self.penalty.alpha * original_cost
                / problem.number_of_features(&prev_param)? as f64;
            accepted = true;
        }

        if accepted {
            self.cur_neighbors = None;
            Ok(state.param(new_state).cost(original_cost))
        } else {
            let prev_cost = state.get_prev_cost();
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
