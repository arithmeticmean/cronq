use clap::{Parser, Subcommand};

pub use super::subcommand::{CancelArgs, JobsArgs, LogsArgs, ScheduleArgs};

/// A reliable, self-hosted cron system. Schedule jobs across your servers.
#[derive(Debug, Parser)]
#[command(name = "cronsys", version, about)]
pub struct CliArgs {
    /// Available commands
    #[command(subcommand)]
    subcommand: CliSubcommand,
}

/// Subcommands for scheduling, managing, and inspecting jobs
#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    /// Schedule a new job
    Schedule(ScheduleArgs),

    /// Show active or past jobs
    Jobs(JobsArgs),

    /// Remove a job by its ID
    Cancel(CancelArgs),

    /// View job execution logs by job ID
    Logs(LogsArgs),
}
impl CliArgs {
    pub fn try_parse_from_env() -> Result<Self, clap::Error> {
        let args = Self::try_parse_from(std::env::args_os())?;
        Ok(args)
    }

    pub fn subcommand(&self) -> &CliSubcommand {
        &self.subcommand
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use shell_words;

    fn parse_args(args: &str) -> Result<CliArgs, clap::Error> {
        let parts = shell_words::split(args).expect("Failed to split shell-style args");
        CliArgs::try_parse_from(parts)
    }

    #[test]
    fn test_create_subcommand_valid() {
        let args = parse_args(
            "appname schedule --when '* * * * * *' --run 'echo hello' --server https://xyz.com -v",
        )
        .unwrap();

        if let CliSubcommand::Schedule(create_args) = args.subcommand {
            assert_eq!(create_args.run(), "echo hello");
            assert_eq!(create_args.when(), "* * * * * *");
            assert_eq!(create_args.server(), "https://xyz.com");
            assert!(create_args.isverbose());
        } else {
            panic!("Expected `schedule` subcommand");
        }
    }

    #[test]
    fn test_create_subcommand_invalid_schedule() {
        let err = parse_args(
            "appname schedule --when '* * * * *' --run 'echo hello' --server https://xyz.com",
        )
        .unwrap_err();
        assert!(
            format!("{}", err).contains("Invalid cron expression"),
            "Expected invalid cron error, got: {}",
            err
        );
    }

    #[test]
    fn test_create_subcommand_missing_server() {
        let err =
            parse_args("appname schedule --when '* * * * * *' --run 'echo hello'").unwrap_err();
        assert!(
            format!("{}", err).contains("--server"),
            "Expected missing server error, got: {}",
            err
        );
    }

    #[test]
    fn test_list_subcommand() {
        let args = parse_args("appname jobs --server https://xyz.com").unwrap();

        if let CliSubcommand::Jobs(list_args) = args.subcommand {
            assert_eq!(list_args.server(), "https://xyz.com");
            assert!(!list_args.isverbose());
        } else {
            panic!("Expected List subcommand");
        }
    }

    #[test]
    fn test_delete_subcommand() {
        let args =
            parse_args("appname cancel --job-id job123 --server https://xyz.com -v").unwrap();

        if let CliSubcommand::Cancel(delete_args) = args.subcommand {
            assert_eq!(delete_args.job_id(), "job123");
            assert_eq!(delete_args.server(), "https://xyz.com");
            assert!(delete_args.isverbose());
        } else {
            panic!("Expected Delete subcommand");
        }
    }

    #[test]
    fn test_logs_subcommand() {
        let args = parse_args("appname logs --job-id job123 --server https://xyz.com").unwrap();

        if let CliSubcommand::Logs(logs_args) = args.subcommand {
            assert_eq!(logs_args.job_id(), "job123");
            assert_eq!(logs_args.server(), "https://xyz.com");
            assert!(!logs_args.isverbose());
        } else {
            panic!("Expected Logs subcommand");
        }
    }
}
