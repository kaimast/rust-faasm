/// Print an error message and terminate the program
#[macro_export]
macro_rules! fatal {
    ($($args:tt)*) => {
        let msg = format!($($args)*);
        println!("{{\"err\":{msg}}}");
        std::process::exit(0);
    }
}

/// Print a log message
#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        let msg = format!($($args)*);
        println!("{{\"info\":{msg}}}");
    }
}

pub use fatal;
pub use info;
