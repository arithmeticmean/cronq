use cron::Schedule;
use reqwest::{Method, header};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use super::config::HttpClientConfig;
use super::endpoint::Endpoint;
use super::error::ClientError;
use super::payload::{RequestPayload, ResponsePayload};
use super::schedule::{JobScheduleRequest, JobScheduleResponse};

pub struct HttpClient {
    client: ClientWithMiddleware,
    server_base_url: Url,
}

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Result<Self, ClientError> {
        let reqwest_client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| ClientError::Init(e.to_string()))?;

        let backoff_policy =
            ExponentialBackoff::builder().build_with_max_retries(config.max_retries);

        let client = ClientBuilder::new(reqwest_client)
            .with(RetryTransientMiddleware::new_with_policy(backoff_policy))
            .build();

        let server_base_url = config.base_url;

        Ok(Self {
            client,
            server_base_url,
        })
    }

    pub async fn schedule(&self, cron: &str, command: &str) -> Result<(), ClientError> {
        let request_payload = JobScheduleRequest::new(cron, command);

        let response: JobScheduleResponse = self
            .post(
                request_payload,
                self.server_base_url.join(Endpoint::Schedule.as_path())?,
            )
            .await?;

        response.print();
        Ok(())
    }

    async fn post<T, R>(&self, request_payload: T, url: Url) -> Result<R, ClientError>
    where
        T: RequestPayload,
        R: ResponsePayload,
    {
        let response = self
            .client
            .request(Method::POST, url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(request_payload.json()?)
            .send()
            .await?;

        let response_status = response.status();
        let response_body = response.text().await?;

        if !response_status.is_success() {
            return Err(ClientError::ApiError {
                status: response_status,
                body: response_body,
            });
        }

        R::from_str(response_body.as_str())
    }

    pub async fn get<R>(&self, endpoint: &str) -> Result<R, ClientError>
    where
        R: ResponsePayload,
    {
        let response = self
            .client
            .request(Method::GET, self.server_base_url.join(endpoint)?)
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        let response_status = response.status();
        let response_body = response.text().await?;

        if !response_status.is_success() {
            return Err(ClientError::ApiError {
                status: response_status,
                body: response_body,
            });
        }

        R::from_str(response_body.as_str())
    }
}
