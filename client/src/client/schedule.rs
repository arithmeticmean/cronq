use super::error::ClientError;
use super::payload::{RequestPayload, ResponsePayload};
use std::time;

use serde::{Deserialize, Serialize};

/// Request to schedule a job.
#[derive(Serialize)]
pub struct JobScheduleRequest<'a, 'b> {
    /// Cron expression.
    pub(super) cron: &'a str,
    /// Command to execute.
    pub(super) command: &'b str,
}

impl<'a, 'b> JobScheduleRequest<'a, 'b> {
    pub fn new(cron: &'a str, command: &'b str) -> Self {
        Self { cron, command }
    }
}

impl<'a, 'b> RequestPayload for JobScheduleRequest<'a, 'b> {
    fn json(&self) -> Result<Vec<u8>, ClientError>
    where
        Self: Sized + Serialize,
    {
        let json = serde_json::to_vec(self)?;
        Ok(json)
    }
}

/// Response after scheduling a job.
#[derive(Deserialize)]
pub struct JobScheduleResponse {
    /// Job ID from the server.
    job_id: String,
    /// Time until next run.
    next_run: time::Duration,
    /// 's' = scheduled, 'f' = failed.
    status: JobScheduleStatus,
}

#[derive(Deserialize)]
enum JobScheduleStatus {
    #[serde(rename = "s")]
    Scheduled,
    #[serde(rename = "f")]
    Failed,
}

impl ResponsePayload for JobScheduleResponse {
    fn from_str(body: &str) -> Result<Self, ClientError> {
        let deserialized = serde_json::from_str(body)?;
        Ok(deserialized)
    }

    fn print(&self) {
        match self.status {
            JobScheduleStatus::Scheduled => println!(
                "Job scheduled successfully, job-id: {}, next-run: {:?}",
                self.job_id, self.next_run
            ),
            JobScheduleStatus::Failed => println!(
                "Job scheduling failed, job-id: {}, next-run: {:?}",
                self.job_id, self.next_run
            ),
        }
    }
}
