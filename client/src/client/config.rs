use std::time;
use url::Url;

use super::error::ClientError;

/// Configuration for an HTTP client, including retry behavior and timeouts.
#[derive(Debug)]
pub struct HttpClientConfig {
    pub(super) base_url: Url,
    pub(super) timeout: time::Duration,
    pub(super) max_retries: u32,
}

pub struct HttpClientConfigBuilder<'a> {
    base_url: &'a str,
    timeout: time::Duration,
    max_retries: u32,
    retry_delay: time::Duration,
}

impl HttpClientConfig {
    /// Creates a new `HttpClientConfigBuilder` with the specified base URL
    /// and default values for other settings.
    pub fn with_base_url<'a>(base_url: &'a str) -> HttpClientConfigBuilder<'a> {
        HttpClientConfigBuilder {
            base_url,
            timeout: time::Duration::from_secs(4),
            max_retries: 3,
            retry_delay: time::Duration::from_secs(1),
        }
    }
}
impl<'a> HttpClientConfigBuilder<'a> {
    /// Builds the `HttpClientConfig` from `HttpClientConfigBuilder`
    pub fn build(self) -> Result<HttpClientConfig, ClientError> {
        Ok(HttpClientConfig {
            base_url: Url::parse(self.base_url)?,
            timeout: self.timeout,
            max_retries: self.max_retries,
        })
    }
    /// Sets the request timeout duration.
    pub fn timeout(mut self, timeout: time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the maximum number of retries for failed requests.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the delay between retry attempts.
    pub fn retry_delay(mut self, retry_delay: time::Duration) -> Self {
        self.retry_delay = retry_delay;
        self
    }
}
