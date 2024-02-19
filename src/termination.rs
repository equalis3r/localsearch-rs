
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum Status {
    Terminated(Reason),
    #[default]
    NotTerminated,
}

impl Status {
    #[must_use]
    pub fn terminated(&self) -> bool {
        matches!(self, Self::Terminated(_))
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Terminated(reason) => write!(f, "{reason}"),
            Self::NotTerminated => write!(f, "Running"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Reason {
    MaxItersReached,
    MaxTimeReached,
    MaxStallBestReached,
    TargetCostReached,
    KeyboardInterrupt,
    SolverConverged,
    SolverExit(String),
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reason = match self {
            Self::MaxItersReached => "Maximum number of iterations reached",
            Self::MaxTimeReached => "Maximum time reached",
            Self::MaxStallBestReached => "Maximum stall best reached",
            Self::TargetCostReached => "Target cost value reached",
            Self::KeyboardInterrupt => "Keyboard interrupt",
            Self::SolverConverged => "Solver converged",
            Self::SolverExit(reason) => reason.as_ref(),
        };
        write!(f, "{reason}")
    }
}

impl Default for Reason {
    fn default() -> Self {
        Self::SolverExit("Undefined".to_owned())
    }
}
