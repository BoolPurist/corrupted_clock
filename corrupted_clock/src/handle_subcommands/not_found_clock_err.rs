use thiserror::Error;

use crate::cli_args::ClockKind;

#[derive(Debug, Error)]
#[error("No {kind} found under the name \"{name}\"")]
pub struct NotFoundClockErr {
    name: String,
    kind: ClockKind,
}

impl NotFoundClockErr {
    pub fn new(value: String, kind: ClockKind) -> Self {
        Self { name: value, kind }
    }
}
