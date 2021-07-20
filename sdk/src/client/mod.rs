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

//! Traits and implementations useful for interacting with the REST API.

// #[cfg(feature = "client-reqwest")]
pub mod agent;
pub mod location;
pub mod organization;
pub mod product;
pub mod purchase_order;
pub mod reqwest;
pub mod role;
pub mod schema;

use crate::error::ClientError;
use sawtooth_sdk::messages::batch::BatchList;

use self::reqwest::ReqwestClient;

pub trait Client {
    /// Submits a list of batches
    ///
    /// # Arguments
    ///
    /// * `batches` - The `BatchList` to be submitted
    fn post_batches(
        &self,
        wait: u64,
        batch_list: &BatchList,
        service_id: Option<&str>,
    ) -> Result<(), ClientError>;
}

//TODO: add a factory here
// look at splinter's database code (store, traits, storeFactory)

pub trait ClientFactory {
    /// Retrieves a client for listing and showing agents
    fn agent_client(url: String) -> Box<dyn agent::AgentClient>;

    /// Retrieves a client for listing and showing schemas
    fn schema_client(url: String) -> Box<dyn schema::SchemaClient>;

    /// Retrieves a client for listing and showing roles
    fn role_client(url: String) -> Box<dyn role::RoleClient>;

    /// Retrieves a client for listing and showing organizations
    fn organization_client(url: String) -> Box<dyn organization::OrganizationClient>;

    /// Retrieves a client for listing and showing locations
    fn location_client(url: String) -> Box<dyn location::LocationClient>;

    /// Retrieves a client for listing and showing products
    fn product_client(url: String) -> Box<dyn product::ProductClient>;

    /// Retrieves a client for listing and showing
    /// purchase orders, revisions, and versions
    #[cfg(feature = "purchase-order")]
    fn purchase_order_client(url: String) -> Box<dyn purchase_order::PurchaseOrderClient>;
}

pub struct HttpClientFactory {}

impl ClientFactory for HttpClientFactory {
    fn agent_client(url: String) -> Box<dyn agent::AgentClient> {
        Box::new(ReqwestClient::new(url))
    }

    fn schema_client(url: String) -> Box<dyn schema::SchemaClient> {
        Box::new(ReqwestClient::new(url))
    }

    fn role_client(url: String) -> Box<dyn role::RoleClient> {
        Box::new(ReqwestClient::new(url))
    }

    fn organization_client(url: String) -> Box<dyn organization::OrganizationClient> {
        Box::new(ReqwestClient::new(url))
    }

    fn location_client(url: String) -> Box<dyn location::LocationClient> {
        Box::new(ReqwestClient::new(url))
    }

    fn product_client(url: String) -> Box<dyn product::ProductClient> {
        Box::new(ReqwestClient::new(url))
    }

    #[cfg(feature = "purchase-order")]
    fn purchase_order_client(url: String) -> Box<dyn purchase_order::PurchaseOrderClient> {
        Box::new(ReqwestClient::new(url))
    }
}
