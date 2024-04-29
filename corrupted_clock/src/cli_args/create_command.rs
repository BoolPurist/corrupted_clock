use clap::Args;
use corrupted_clock_util::timing::ClockDuration;

#[derive(Debug, Args)]
pub struct CreateCommand {
    name: Option<String>,
    #[arg(short, long)]
    /// # Syntax for option value
    ///
    /// Syntax for number: [0-9]+ => sequence of digits from 0 to 9
    ///
    /// Valid syntax for duration for hours, minutes and seconds:
    ///
    /// <number>[:<number>][:<number>]
    ///
    /// Valid syntax for duration for minutes and seconds:
    ///
    /// <number>[:<number>]
    ///
    /// Valid syntax for duration for seconds:
    ///
    /// <number>
    ///
    /// # Examples
    ///
    /// Duration with 12 hours, 56 minutes and 12 seconds
    ///
    /// 12:56:12
    ///
    /// Duration with 56 minutes and 12 seconds
    ///
    /// 56:12
    ///
    /// Duration with 12 seconds
    ///
    /// 12
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
