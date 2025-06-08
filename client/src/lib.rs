#![allow(unused_variables)]
mod cli;
mod client;
mod error;
mod utils;

use cli::{CliArgs, CliSubcommand};
use client::{HttpClient, HttpClientConfig};
use error::AppError;

pub async fn run_app() -> Result<(), AppError> {
    let args = CliArgs::try_parse_from_env()?;

    match args.subcommand() {
        CliSubcommand::Schedule(schedule_args) => HttpClient::new(
            HttpClientConfig::with_base_url(schedule_args.server())
                .max_retries(2)
                .build()?,
        )?
        .schedule(schedule_args.when(), schedule_args.run())
        .await
        .map_err(|e| e.into()),

        CliSubcommand::Logs(logs_args) => todo!(),

        CliSubcommand::Jobs(jobs_args) => todo!(),

        CliSubcommand::Cancel(cancel_args) => todo!(),
    }
}
