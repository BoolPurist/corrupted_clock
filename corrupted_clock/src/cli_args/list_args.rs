use clap::Args;

use crate::AtLeastOne;

use super::{ClockKind, ColumnShowArg};

#[derive(Debug, Args)]
pub struct ListArgs {
    #[arg(short, long)]
    kind: Option<ClockKind>,
    #[command(flatten)]
    colums_num: ColumnShowArg,
}

impl ListArgs {
    pub fn kind(&self) -> Option<ClockKind> {
        self.kind
    }

    pub fn colums_num(&self) -> Option<AtLeastOne> {
        self.colums_num.colums_num()
    }
}
