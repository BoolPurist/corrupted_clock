use clap::Args;

use crate::AtLeastOne;

use super::{ClockReference, ColumnShowArg};

#[derive(Debug, Args)]
pub struct GetClockArgs {
    #[command(flatten)]
    reference: ClockReference,
    #[command(flatten)]
    column_num: ColumnShowArg,
}

impl GetClockArgs {
    pub fn reference(&self) -> &ClockReference {
        &self.reference
    }

    pub fn column_num(&self) -> Option<AtLeastOne> {
        self.column_num.colums_num()
    }
}
