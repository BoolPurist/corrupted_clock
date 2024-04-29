use clap::ValueEnum;

use crate::constants;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Default, ValueEnum)]
pub enum ManyClockReferenceKind {
    #[default]
    #[value(alias(constants::ALL_CLOCK_ALIASE))]
    All,
    #[value(alias(constants::STOP_WATCH_ALIASE))]
    Stopwatch,
    #[value(alias(constants::COUNT_DOWN_ALIASE))]
    CountDown,
}
