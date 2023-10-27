use std::error;
use std::fmt;

#[derive(Debug)]
pub enum LocalSearchError {
    NotInitialized,
    FailGenRandomState,
    FailGenCandidateState,
    Bug,
}

impl error::Error for LocalSearchError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            _ => None,
        }
    }
}

impl fmt::Display for LocalSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LocalSearchError::NotInitialized => {
                write!(f, "Fail to initialize state parameters")
            }
            LocalSearchError::FailGenRandomState => {
                write!(f, "Fail to generate a random state")
            }
            LocalSearchError::FailGenCandidateState => {
                write!(f, "Fail to generate a candidate state")
            }
            LocalSearchError::Bug => {
                write!(f, "Bug")
            }
        }
    }
}
