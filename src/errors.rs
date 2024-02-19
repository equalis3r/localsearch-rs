use std::fmt;

#[derive(Debug)]
pub enum LocalSearchError {
    NotInitialized,
    FailGenRandomState,
    FailGenCandidateState,
    Bug,
}

impl fmt::Display for LocalSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotInitialized => {
                write!(f, "Fail to initialize state parameters")
            }
            Self::FailGenRandomState => {
                write!(f, "Fail to generate a random state")
            }
            Self::FailGenCandidateState => {
                write!(f, "Fail to generate a candidate state")
            }
            Self::Bug => {
                write!(f, "Bug")
            }
        }
    }
}
