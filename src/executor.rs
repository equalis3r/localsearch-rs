use crate::errors::LocalSearchError;
use crate::result::LocalSearchSolution;
use crate::solver::Solver;
use crate::state::State;
use crate::termination::Status;
use std::time;

pub struct Executor<O, S, I> {
    solver: S,
    problem: O,
    state: Option<I>,
    duration: time::Duration,
}

impl<O, S, I> Executor<O, S, I>
where
    S: Solver<O, I>,
    I: State,
{
    pub fn new(problem: O, solver: S) -> Self {
        Self {
            solver,
            problem,
            state: Some(I::new()),
            duration: time::Duration::new(0, 0),
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if
    pub fn configure<F: FnOnce(I) -> I>(mut self, init: F) -> Result<Self, LocalSearchError> {
        match self.state.take() {
            None => Err(LocalSearchError::NotInitialized),
            Some(state) => {
                self.state = Some(init(state));
                Ok(self)
            }
        }
    }

    /// # Panics
    ///
    /// Panic if
    /// # Errors
    ///
    /// Will return `Err` if
    pub fn run(mut self) -> Result<LocalSearchSolution<O, S, I>, LocalSearchError> {
        let total_time = time::Instant::now();

        let Some(state) = self.state.take() else {
            return Err(LocalSearchError::NotInitialized);
        };

        let mut state = self.solver.init(&mut self.problem, state)?;
        state.update();

        loop {
            state = if state.terminated() {
                state
            } else if let Status::Terminated(reason) = self.solver.terminate_internal(&state) {
                state.terminate_with(reason)
            } else {
                state
            };

            if state.terminated() {
                break;
            }

            state = self.solver.next_iter(&mut self.problem, state)?;
            state.update();
            state.increment_iter();

            if state.terminated() {
                break;
            }
        }
        self.duration = total_time.elapsed();

        Ok(LocalSearchSolution::new(self.problem, self.solver, state))
    }
}
