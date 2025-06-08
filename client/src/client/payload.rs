use super::error::ClientError;
use serde::{Serialize, de::DeserializeOwned};

pub trait ResponsePayload: DeserializeOwned + Sized {
    /// Construct Self from a raw HTTP response body.
    fn from_str(body: &str) -> Result<Self, ClientError>;

    /// Prints the response.
    fn print(&self);
}

pub trait RequestPayload: Serialize {
    /// Construct Self from a type T
    fn json(&self) -> Result<Vec<u8>, ClientError>
    where
        Self: Serialize + Sized;
}
