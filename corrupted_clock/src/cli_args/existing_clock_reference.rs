use clap::Args;
use thiserror::Error;

use super::{ClockKind, ClockReference, ManyClockReferenceKind};

#[derive(Debug, Args)]
/// Note: You can either provide the positional parameter `name` and
/// the option `kind` or the option `all`.
pub struct ExistingClockReference {
    /// Does the action on a single stopwatch or count down
    name: Option<String>,
    #[arg(short, long)]
    /// Specifies if the action performed on a stopwatch or count down.
    kind: Option<ClockKind>,
    #[arg(short, long)]
    /// Does the action on all stopwatches, count downs or both
    all: Option<ManyClockReferenceKind>,
}

#[derive(Debug)]
pub enum ExistingClockKindReference {
    All(ManyClockReferenceKind),
    Single(ClockReference),
}

#[derive(Debug, Error)]
pub enum InvalidArgsClockReference {
    #[error("No name is allowed if the (all) flat is set")]
    AllWithName,
    #[error("A name to a stopwatch or count down needs to be provided without the (all) flat set")]
    NoNameForSingle,
}

impl ExistingClockReference {
    pub fn kind_reference(&self) -> Result<ExistingClockKindReference, InvalidArgsClockReference> {
        let flags_for_single = self.name.is_some() || self.kind.is_some();
        match (self.all, flags_for_single) {
            (Some(_), true) => return Err(InvalidArgsClockReference::AllWithName),
            (None, false) => return Err(InvalidArgsClockReference::NoNameForSingle),
            _ => (),
        }

        let reference = self
            .name
            .as_deref()
            .map(|single| {
                ExistingClockKindReference::Single(ClockReference::new(
                    single.to_owned(),
                    self.kind,
                ))
            })
            .unwrap_or_else(|| {
                ExistingClockKindReference::All(
                    self.all
                        .expect("None case is ruled out the start of the function by the if guard"),
                )
            });
        Ok(reference)
    }
}
