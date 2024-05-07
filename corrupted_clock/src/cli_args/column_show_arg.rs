use clap::Args;

use crate::AtLeastOne;

#[derive(Debug, Args)]
pub struct ColumnShowArg {
    #[arg(short, long)]
    /// How many colums are printed within a row at max
    ///
    /// Note: Must be greater than zero
    colums_num: Option<AtLeastOne>,
}

impl ColumnShowArg {
    pub fn colums_num(&self) -> Option<AtLeastOne> {
        self.colums_num
    }
}
