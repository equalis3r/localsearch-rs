pub mod guided_local_search;
pub mod tabu_search;
pub mod variable_neighborhood;

pub use guided_local_search::GuidedLocalSearch;
pub use tabu_search::TabuSearch;
pub use variable_neighborhood::VariableNeighborhood;

use crate::errors::LocalSearchError;
use crate::state::State;
use crate::termination::{Reason, Status};

pub trait Solver<O, I: State> {
    const NAME: &'static str;

    /// # Errors
    ///
    /// Will return `Err` if
    fn init(&mut self, _problem: &mut O, state: I) -> Result<I, LocalSearchError> {
        Ok(state)
    }

    /// # Errors
    ///
    /// Will return `Err` if
    fn next_iter(&mut self, problem: &mut O, state: I) -> Result<I, LocalSearchError>;

    fn terminate_internal(&mut self, state: &I) -> Status {
        let solver_status = self.terminate();
        if solver_status.terminated() {
            return solver_status;
        }
        if state.get_max_iters() <= state.get_iter() {
            return Status::Terminated(Reason::MaxItersReached);
        }
        if state.get_best_cost() <= state.get_target_cost() {
            return Status::Terminated(Reason::TargetCostReached);
        }
        Status::NotTerminated
    }

    fn terminate(&mut self) -> Status {
        Status::NotTerminated
    }
}
