#[macro_export]
macro_rules! env_var_prefix {
    ($suffix:literal) => {{
        concat!("CORRUPTED_CLOCK_", $suffix)
    }};
}

pub use env_var_prefix;
