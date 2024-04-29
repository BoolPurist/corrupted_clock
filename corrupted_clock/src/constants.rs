#[macro_export]
macro_rules! env_var_prefix {
    ($suffix:literal) => {{
        concat!("CORRUPTED_CLOCK_", $suffix)
    }};
}

pub use env_var_prefix;

pub const STOP_WATCH_ALIASE: &str = "sw";
pub const COUNT_DOWN_ALIASE: &str = "cd";
pub const ALL_CLOCK_ALIASE: &str = "a";
