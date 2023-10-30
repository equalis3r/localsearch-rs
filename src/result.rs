use crate::problem::Problem;
use crate::solver::Solver;
use crate::state::State;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone)]
pub struct MetaSolution<O, S, I> {
    pub problem: Problem<O>,
    pub solver: S,
    pub state: I,
}

impl<O, S, I> MetaSolution<O, S, I> {
    pub fn new(problem: Problem<O>, solver: S, state: I) -> Self {
        Self {
            problem,
            solver,
            state,
        }
    }

    pub fn problem(&self) -> &Problem<O> {
        &self.problem
    }

    pub fn solver(&self) -> &S {
        &self.solver
    }

    pub fn state(&self) -> &I {
        &self.state
    }
}

impl<O, S, I> fmt::Display for MetaSolution<O, S, I>
where
    I: State,
    I::Param: fmt::Debug,
    S: Solver<O, I>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "OptimizationResult:")?;
        writeln!(f, "    Solver:        {}", S::NAME)?;
        writeln!(
            f,
            "    param (best):  {}",
            self.state.get_best_param().map_or_else(
                || String::from("None"),
                |best_param| format!("{best_param:?}")
            )
        )?;
        writeln!(f, "    cost (best):   {}", self.state.get_best_cost())?;
        writeln!(f, "    iters (best):  {}", self.state.get_last_best_iter())?;
        writeln!(f, "    iters (total): {}", self.state.get_iter())?;
        writeln!(
            f,
            "    termination:   {}",
            self.state.get_termination_status()
        )?;
        if let Some(time) = self.state.get_time() {
            writeln!(f, "    time:          {time:?}")?;
        }
        Ok(())
    }
}

impl<O, S, I: State> PartialEq for MetaSolution<O, S, I> {
    /// Two `OptimizationResult`s are equal if the absolute of the difference between their best
    /// cost values is smaller than epsilon.
    fn eq(&self, other: &Self) -> bool {
        (self.state.get_best_cost() - other.state.get_best_cost()).abs() < f64::EPSILON
    }
}

impl<O, S, I: State> Eq for MetaSolution<O, S, I> {}

impl<O, S, I: State> Ord for MetaSolution<O, S, I> {
    /// Two `OptimizationResult`s are equal if the absolute of the difference between their best
    /// cost values is smaller than epsilon.
    /// Else, an `OptimizationResult` is better if the best cost function value is strictly better
    /// than the others.
    fn cmp(&self, other: &Self) -> Ordering {
        let t = self.state.get_best_cost() - other.state.get_best_cost();
        if t.abs() < f64::EPSILON {
            Ordering::Equal
        } else if t > 0.0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl<O, S, I: State> PartialOrd for MetaSolution<O, S, I> {
    /// Two `OptimizationResult`s are equal if the absolute of the difference between their best
    /// cost values is smaller than epsilon.
    /// Else, an `OptimizationResult` is better if the best cost function value is strictly better
    /// than the others.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
