pub mod iteration;

use crate::termination::{Reason, Status};
pub use iteration::IterState;
use std::time;

pub trait State {
    type Param;

    fn new() -> Self;

    /// This method is called after each iteration and checks if the new parameter vector
    /// is better than the previous one. If so, it will update the current best parameter
    /// vector and current best cost function value.
    fn update(&mut self);

    fn get_param(&self) -> Option<&Self::Param>;

    fn get_best_param(&self) -> Option<&Self::Param>;

    fn get_max_iters(&self) -> u32;

    fn increment_iter(&mut self);

    fn get_iter(&self) -> u32;

    fn get_cost(&self) -> f64;

    fn get_best_cost(&self) -> f64;

    fn get_target_cost(&self) -> f64;

    /// Set time required since the beginning of the optimization until current iteration
    fn time(&mut self, time: Option<time::Duration>) -> &mut Self;

    /// Get time passed since the beginning of the optimization until current iteration
    fn get_time(&self) -> Option<time::Duration>;

    fn get_prev_best_iter(&self) -> u32;

    fn is_best(&self) -> bool;

    #[must_use]
    fn terminate_with(self, termination_reason: Reason) -> Self;

    fn get_termination_status(&self) -> &Status;

    fn get_termination_reason(&self) -> Option<&Reason>;

    fn terminated(&self) -> bool {
        matches!(self.get_termination_status(), Status::Terminated(_))
    }
}
