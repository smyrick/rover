use thiserror::Error;

use std::io;

/// RoverupError is the type of Error that occured.
#[derive(Error, Debug)]
pub enum RoverupError {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("Aborting installation")]
    AbortInstall,
}
