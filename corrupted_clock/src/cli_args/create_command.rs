use clap::Args;
use corrupted_clock_util::timing::ClockDuration;

#[derive(Debug, Args)]
pub struct CreateCommand {
    name: Option<String>,
    #[arg(short, long)]
    to_count_down: Option<ClockDuration>,
}

impl CreateCommand {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn to_count_down(&self) -> Option<ClockDuration> {
        self.to_count_down
    }
}
