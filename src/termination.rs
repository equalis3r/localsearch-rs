use core::fmt;

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
            Self::Terminated(reason) => f.write_str(reason.text()),
            Self::NotTerminated => f.write_str("Running"),
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

impl Reason {
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Self::MaxItersReached => "Maximum number of iterations reached",
            Self::MaxTimeReached => "Maximum time reached",
            Self::MaxStallBestReached => "Maximum stall best reached",
            Self::TargetCostReached => "Target cost value reached",
            Self::KeyboardInterrupt => "Keyboard interrupt",
            Self::SolverConverged => "Solver converged",
            Self::SolverExit(reason) => reason.as_ref(),
        }
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl Default for Reason {
    fn default() -> Self {
        Self::SolverExit("Undefined".to_owned())
    }
}
