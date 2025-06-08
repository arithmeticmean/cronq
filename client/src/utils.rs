use ansi_term::Colour::{Red, Yellow};

use std::process;

pub fn exit_with_code(code: i32) -> ! {
    process::exit(code);
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        eprintln!("{}: {}", Yellow.paint("warning"), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        println!("{}", format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! unwrap_or_die{
    ($expr:expr, $($arg:tt)*) => {{
        match $expr {
            Ok(val) => val,
            Err(err) => {
                die!("{}: {}", format!($($arg)*), err);
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_or_die {
    ($cond:expr, $($arg:tt)*) => {{
        if !$cond {
            die!($($arg)*);
        }
    }};
}
