use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Copy, Debug)]
pub struct AtLeastOne(u32);

#[derive(Debug, Error)]
#[error("Value must not be zero")]
pub struct IsZeroError;

#[derive(Debug, Error)]
pub enum ParseNonPosZeroError {
    #[error("Value must be a postive number")]
    NoPostiveNumber,
    #[error("{0}")]
    IsZero(#[from] IsZeroError),
}

impl FromStr for AtLeastOne {
    type Err = ParseNonPosZeroError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos_number: u32 = s
            .parse()
            .map_err(|_| ParseNonPosZeroError::NoPostiveNumber)?;
        let value = AtLeastOne::new(pos_number)?;
        Ok(value)
    }
}

impl AtLeastOne {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn new(value: u32) -> Result<Self, IsZeroError> {
        if value == 0 {
            Err(IsZeroError)
        } else {
            Ok(Self(value))
        }
    }
}
