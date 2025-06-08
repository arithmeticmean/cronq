pub enum Endpoint {
    Schedule,
    Jobs,
    Logs,
    Cancel,
}

impl Endpoint {
    pub fn as_path(&self) -> &'static str {
        match self {
            Endpoint::Schedule => "schedule",
            Endpoint::Jobs => "jobs",
            Endpoint::Logs => "logs",
            Endpoint::Cancel => "cancel",
        }
    }
}
