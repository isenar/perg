/// Helper macro for skipping `Result::Err` values in a loop
#[macro_export]
macro_rules! skip_fail {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => continue,
        }
    };
}

/// Helper function to check whether perg is currently being piped to.
pub fn is_stdin_piped() -> bool {
    atty::isnt(atty::Stream::Stdin)
}
