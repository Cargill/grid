// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A Reqwest-based implementation of Client

pub mod fetch_methods;
mod reqwest_agent;
mod reqwest_location;
mod reqwest_organization;
mod reqwest_product;
mod reqwest_purchase_order;
mod reqwest_role;
mod reqwest_schema;

use protobuf::Message;
use reqwest::blocking::Client as BlockingClient;
use sawtooth_sdk::messages::batch::BatchList;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Instant;

use super::Client;
use crate::error::ClientError;

#[derive(Deserialize)]
struct ServerError {
    pub message: String,
}

#[derive(Default, Clone)]
pub struct ReqwestClient {
    pub url: String,
}

impl ReqwestClient {
    pub fn new(url: String) -> Self {
        ReqwestClient { url }
    }
}

impl Client for ReqwestClient {
    /// Submits a list of batches
    fn post_batches(
        &self,
        wait: u64,
        batch_list: &BatchList,
        service_id: Option<&str>,
    ) -> Result<(), ClientError> {
        let bytes = batch_list.write_to_bytes().map_err(|_err| {
            ClientError::DaemonError("Failed to convert batch list to bytes".to_string())
        })?;

        let mut wait_time = wait;

        let mut url = format!("{}/batches", self.url);

        if let Some(service_id) = service_id {
            url.push_str(&format!("?service_id={}", service_id));
        }

        let client = BlockingClient::new();

        let response = client
            .post(&url)
            .header("GridProtocolVersion", "1")
            .body(bytes)
            .send()
            .map_err(|_err| ClientError::DaemonError("Failed to post batch list".to_string()))?;

        if !response.status().is_success() {
            return Err(ClientError::DaemonError(response.text().map_err(|_| {
                ClientError::DaemonError("Unable to convert error response to text".to_string())
            })?));
        }

        let batch_link = response.json::<BatchStatusLink>().map_err(|_err| {
            ClientError::DaemonError("Unable to get batch status link from response".to_string())
        })?;

        let params: Vec<&str> = batch_link.link.split('?').collect();

        let id_param: Vec<&str> = params[1].split('=').collect();

        let id = id_param[1];

        info!("Submitted batch: {}", id);

        while wait_time > 0 {
            let time = Instant::now();

            let url = if let Some(service_id) = service_id {
                format!(
                    "{}&wait={}&service_id={}",
                    batch_link.link, wait_time, service_id
                )
            } else {
                format!("{}&wait={}", batch_link.link, wait_time)
            };

            let response = client.get(&url).send().map_err(|_err| {
                ClientError::DaemonError("Unable to get batch status".to_string())
            })?;

            if !response.status().is_success() {
                return Err(ClientError::DaemonError(response.text().map_err(|_| {
                    ClientError::DaemonError("Unable to convert error response to text".to_string())
                })?));
            }

            let batch_status = response.json::<BatchStatusResponse>().map_err(|_err| {
                ClientError::DaemonError("Unable to get batch status response".to_string())
            })?;

            for t in &batch_status.data {
                if t.status == "Invalid" {
                    for i in &t.invalid_transactions {
                        error!(
                            "Error: {}",
                            i.get("message")
                                .unwrap_or(&"Batch contained invalid transactions".to_string())
                        );
                    }
                }
            }

            if batch_status.data.iter().all(|d| d.status == "Valid") {
                info!("Batch and transaction structure was valid. Batch queued.");
            }

            if batch_status.data.iter().all(|x| x.status != "PENDING") {
                break;
            }

            wait_time -= time.elapsed().as_secs()
        }

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct BatchStatusLink {
    pub link: String,
}

#[derive(Deserialize, Debug)]
struct BatchStatusResponse {
    pub data: Vec<BatchStatus>,
    pub link: String,
}

#[derive(Deserialize, Debug)]
struct BatchStatus {
    pub id: String,
    pub invalid_transactions: Vec<HashMap<String, String>>,
    pub status: String,
}
