use clap::Args;

use super::ClockReference;

#[derive(Debug, Args)]
pub struct DeleteArgs {
    #[command(flatten)]
    reference: ClockReference,
}

impl DeleteArgs {
    pub fn reference(&self) -> &ClockReference {
        &self.reference
    }
}
