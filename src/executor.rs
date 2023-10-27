use crate::errors::LocalSearchError;
use crate::problem::Problem;
use crate::result::MetaSolution;
use crate::solver::Solver;
use crate::state::State;
use crate::termination::Status;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Executor<O, S, I> {
    solver: S,
    problem: Problem<O>,
    state: Option<I>,
    timer: bool,
}

impl<O, S, I> Executor<O, S, I>
where
    S: Solver<O, I>,
    I: State,
{
    pub fn new(problem: O, solver: S) -> Self {
        let state = Some(I::new());
        Self {
            solver,
            problem: Problem::new(problem),
            state,
            timer: true,
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
    pub fn run(mut self) -> Result<MetaSolution<O, S, I>, LocalSearchError> {
        let total_time = if self.timer {
            Some(std::time::Instant::now())
        } else {
            None
        };

        let Some(state) = self.state.take() else {
            return Err(LocalSearchError::NotInitialized);
        };
        let interrupt = Arc::new(AtomicBool::new(false));

        // Only call `init` of `solver` if the current iteration number is 0. This avoids that
        // `init` is called when starting from a checkpoint (because `init` could change the state
        // of the `solver`, which would overwrite the state restored from the checkpoint).
        let mut state = if state.get_iter() == 0 {
            let mut state = self.solver.init(&mut self.problem, state)?;
            state.update();
            state
        } else {
            state
        };

        while !interrupt.load(Ordering::SeqCst) {
            // check first if it has already terminated
            // This should probably be solved better.
            // First, check if it isn't already terminated. If it isn't, evaluate the
            // stopping criteria. If `self.terminate()` is called without the checking
            // whether it has terminated already, then it may overwrite a termination set
            // within `next_iter()`!
            state = if state.terminated() {
                state
            } else {
                let term = self.solver.terminate_internal(&state);
                if let Status::Terminated(reason) = term {
                    state.terminate_with(reason)
                } else {
                    state
                }
            };
            // Now check once more if the algorithm has terminated. If yes, then break.
            if state.terminated() {
                break;
            }

            // Start time measurement
            let start = if self.timer {
                Some(std::time::Instant::now())
            } else {
                None
            };

            let state_t = self.solver.next_iter(&mut self.problem, state)?;
            state = state_t;

            // End time measurement
            let _duration = if self.timer {
                Some(start.unwrap().elapsed())
            } else {
                None
            };

            state.update();
            state.increment_iter();

            if self.timer {
                total_time.map(|total_time| state.time(Some(total_time.elapsed())));
            }

            if state.terminated() {
                break;
            }
        }

        Ok(MetaSolution::new(self.problem, self.solver, state))
    }

    #[must_use]
    pub fn timer(mut self, timer: bool) -> Self {
        self.timer = timer;
        self
    }
}
