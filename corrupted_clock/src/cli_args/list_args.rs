use clap::Args;

use super::ClockKind;

#[derive(Debug, Args)]
pub struct ListArgs {
    #[arg(short, long)]
    kind: Option<ClockKind>,
}

impl ListArgs {
    pub fn kind(&self) -> Option<ClockKind> {
        self.kind
    }
}
