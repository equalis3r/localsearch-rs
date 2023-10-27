use thiserror::Error;
#[derive(Error, Debug)]
pub enum MetaError {
    #[error("Fail to initialize state parameters")]
    NotInitialized,
    #[error("Fail to generate a random state")]
    FailGenRandomState,
    #[error("Fail to generate a candidate state")]
    FailGenCandidateState,
    #[error("Bug")]
    Bug,
}
