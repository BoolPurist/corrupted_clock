use clap::Args;

use super::ClockKind;

#[derive(Debug, Args, Clone)]
#[group(required = false, multiple = true)]
pub struct ClockReference {
    name: String,
    #[arg(short, long)]
    kind: Option<ClockKind>,
}

impl ClockReference {
    pub fn new(name: String, kind: Option<ClockKind>) -> Self {
        Self { name, kind }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> ClockKind {
        self.kind.unwrap_or_default()
    }
}
