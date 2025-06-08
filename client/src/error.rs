use crate::client::ClientError;
use ansi_term::Colour::Red;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Cli(#[from] clap::Error),

    #[error(transparent)]
    Client(#[from] ClientError),
}

macro_rules! die {
    ($($arg:tt)*) => {{
        eprintln!("{}: {}", Red.paint("error"), format!($($arg)*));
        $crate::utils::exit_with_code(10);
    }};
}

impl AppError {
    pub fn exit(&self) -> ! {
        match self {
            Self::Cli(err) => err.exit(),
            Self::Client(err) => {
                die!("{}", err)
            }
        }
    }
}
