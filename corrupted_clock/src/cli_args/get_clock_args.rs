use clap::Args;

use super::ClockReference;

#[derive(Debug, Args)]
pub struct GetClockArgs {
    #[command(flatten)]
    reference: ClockReference,
}

impl GetClockArgs {
    pub fn reference(&self) -> &ClockReference {
        &self.reference
    }
}
