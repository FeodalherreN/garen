use super::http_methods::Methods;
use crate::{models::errors::RiftApiRequestError, utilities::lockfile_fetcher::LockfileContents};

use log::debug;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde::Serialize as SerializeDerive;

#[derive(Debug, Clone)]
pub struct RequestClient {
    pub http: Client,
    pub lockfile: LockfileContents,
}

#[derive(SerializeDerive)]
struct EmptyGetRequestBody {}

impl RequestClient {
    pub async fn request<T: DeserializeOwned, S: Serialize + Sized>(
        &mut self,
        method: Methods,
        path: &str,
        body: Option<S>,
    ) -> Result<T, RiftApiRequestError> {
        let url = format!(
            "{}://127.0.0.1:{}/{}",
            self.lockfile.protocol, self.lockfile.port, path
        );

        debug!("Constructed url for {:?} request: {}", method, url);
        let request = self
            .http
            .request(method.as_reqwest(), url)
            .basic_auth("riot", Some(self.lockfile.password.clone()));
        let response = match body {
            Some(b) => request.json(&b).send().await,
            None => request.send().await,
        };
        return match response {
            Ok(r) => {
                if r.content_length().to_owned().unwrap() == 0 {
                    // Return empty?
                }

                let parsed = r.json::<T>().await;
                match parsed {
                    Ok(p) => Ok(p),
                    Err(e) => Err(RiftApiRequestError::new(e)),
                }
            }
            Err(e) => Err(RiftApiRequestError::new(e)),
        };
    }

    pub async fn get<T: DeserializeOwned>(&mut self, path: &str) -> Result<T, RiftApiRequestError> {
        return self
            .request::<T, EmptyGetRequestBody>(Methods::Get, path, None)
            .await;
    }

    pub async fn post<T: DeserializeOwned, S: Serialize + Sized>(
        &mut self,
        path: &str,
        body: Option<S>,
    ) -> Result<T, RiftApiRequestError> {
        return self.request::<T, S>(Methods::Post, path, body).await;
    }

    pub async fn put<T: DeserializeOwned, S: Serialize + Sized>(
        &mut self,
        path: &str,
        body: Option<S>,
    ) -> Result<T, RiftApiRequestError> {
        return self.request::<T, S>(Methods::Put, path, body).await;
    }

    pub async fn delete<T: DeserializeOwned, S: Serialize + Sized>(
        &mut self,
        path: &str,
        body: Option<S>,
    ) -> Result<T, RiftApiRequestError> {
        return self.request::<T, S>(Methods::Delete, path, body).await;
    }

    pub async fn head<T: DeserializeOwned, S: Serialize + Sized>(
        &mut self,
        path: &str,
    ) -> Result<T, RiftApiRequestError> {
        return self.request::<T, S>(Methods::Head, path, None).await;
    }
}

pub fn get_request_client(lockfile: LockfileContents) -> RequestClient {
    let http = Client::builder()
        .add_root_certificate(super::certificate::get_certificate())
        .build()
        .unwrap();
    return RequestClient { http, lockfile };
}
