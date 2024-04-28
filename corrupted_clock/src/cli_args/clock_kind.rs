use clap::{Args, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum ClockKind {
    /// alias: sw
    #[value(alias("sw"))]
    #[default]
    StopWatch,
    /// alias: cd
    #[value(alias("cd"))]
    CountDown,
}

impl std::fmt::Display for ClockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClockKind::StopWatch => f.write_str("stop watch"),
            ClockKind::CountDown => f.write_str("count down"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Args)]
pub struct ClockKindArg {
    #[arg(short, long)]
    kind: Option<ClockKind>,
}

impl ClockKindArg {
    pub fn kind(&self) -> ClockKind {
        self.kind.unwrap_or_default()
    }
}
