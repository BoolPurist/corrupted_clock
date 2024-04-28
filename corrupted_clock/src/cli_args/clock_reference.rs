use clap::Args;

use super::ClockKind;

#[derive(Debug, Args)]
pub struct ClockReference {
    name: String,
    #[arg(short, long)]
    kind: Option<ClockKind>,
}

impl ClockReference {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> ClockKind {
        self.kind.unwrap_or_default()
    }
}
