use crate::solver::Solver;
use crate::state::State;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone)]
pub struct LocalSearchSolution<O, S, I> {
    pub problem: O,
    pub solver: S,
    pub state: I,
}

impl<O, S, I> LocalSearchSolution<O, S, I> {
    pub fn new(problem: O, solver: S, state: I) -> Self {
        Self {
            problem,
            solver,
            state,
        }
    }

    pub fn problem(&self) -> &O {
        &self.problem
    }

    pub fn solver(&self) -> &S {
        &self.solver
    }

    pub fn state(&self) -> &I {
        &self.state
    }

    pub fn take_result(self) -> (O, S, I) {
        (self.problem, self.solver, self.state)
    }
}

impl<O, S, I: State> PartialEq for LocalSearchSolution<O, S, I> {
    fn eq(&self, other: &Self) -> bool {
        (self.state.get_best_cost() - other.state.get_best_cost()).abs() < f64::EPSILON
    }
}

impl<O, S, I: State> Eq for LocalSearchSolution<O, S, I> {}

impl<O, S, I: State> Ord for LocalSearchSolution<O, S, I> {
    fn cmp(&self, other: &Self) -> Ordering {
        let t = self.state.get_best_cost() - other.state.get_best_cost();
        if t.abs() < f64::EPSILON {
            Ordering::Equal
        } else if t.is_sign_positive() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl<O, S, I: State> PartialOrd for LocalSearchSolution<O, S, I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<O, S, I> fmt::Display for LocalSearchSolution<O, S, I>
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
        writeln!(f, "    iters (best):  {}", self.state.get_prev_best_iter())?;
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
