use super::globals;
use anyhow::{Context, Result};
use clap::Parser;
use cron::Schedule;
use std::str::FromStr;

/// Arguments for the `delete` command
#[derive(Debug, Parser)]
pub struct CancelArgs {
    /// ID of the job to delete
    #[arg(long, short)]
    job_id: String,

    #[command(flatten)]
    globals: globals::GlobalArgs,
}

/// Arguments for the `list` command
#[derive(Debug, Parser)]
pub struct JobsArgs {
    #[command(flatten)]
    globals: globals::GlobalArgs,
}

/// Arguments for the `logs` command
#[derive(Debug, Parser)]
pub struct LogsArgs {
    /// ID of the job to view logs for
    #[arg(long, short)]
    job_id: String,

    #[command(flatten)]
    globals: globals::GlobalArgs,
}

/// Arguments for the `create` command
#[derive(Debug, Parser)]
pub struct ScheduleArgs {
    /// Extended Cron time expression i.e 6-fields (e.g., "0 * * * * *")
    #[arg(long, short, value_parser = Self::cron_expression_validator)]
    when: String,

    /// Command to run at the scheduled time
    #[arg(long, short)]
    run: String,

    #[clap(flatten)]
    globals: globals::GlobalArgs,
}

impl ScheduleArgs {
    pub fn cron_expression_validator(cron: &str) -> Result<String> {
        Schedule::from_str(cron).context("Invalid cron expression")?;
        Ok(cron.to_string())
    }
    pub fn isverbose(&self) -> bool {
        self.globals.verbose
    }
    pub fn server(&self) -> &str {
        &self.globals.server
    }
    pub fn run(&self) -> &str {
        self.run.as_str()
    }
    pub fn when(&self) -> &str {
        self.when.as_str()
    }
}

impl CancelArgs {
    pub fn job_id(&self) -> &str {
        self.job_id.as_str()
    }

    pub fn server(&self) -> &str {
        &self.globals.server
    }

    pub fn isverbose(&self) -> bool {
        self.globals.verbose
    }
}

impl JobsArgs {
    pub fn server(&self) -> &str {
        &self.globals.server
    }

    pub fn isverbose(&self) -> bool {
        self.globals.verbose
    }
}

impl LogsArgs {
    pub fn job_id(&self) -> &str {
        self.job_id.as_str()
    }

    pub fn server(&self) -> &str {
        &self.globals.server
    }

    pub fn isverbose(&self) -> bool {
        self.globals.verbose
    }
}
