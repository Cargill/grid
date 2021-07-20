use super::fetch_methods::{fetch_entities_list, fetch_entity};
use crate::client::schema::{GridSchema, SchemaClient};
use crate::error::ClientError;

use super::ReqwestClient;

const SCHEMA_ROUTE: &str = "schema";

impl SchemaClient for ReqwestClient {
    /// Fetches a single schema based on name
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the schema (identifier)
    /// * `service_id` - optional - the service id to fetch the schema from
    fn get_schema(
        &self,
        name: String,
        service_id: Option<&str>,
    ) -> Result<GridSchema, ClientError> {
        fetch_entity::<GridSchema>(&self.url, format!("{}/{}", SCHEMA_ROUTE, name), service_id)
    }

    /// Fetches a list of schemas for the organization
    ///
    /// # Arguments
    ///
    /// * `service_id` - optional - the service id to fetch the schemas from
    fn list_schemas(&self, service_id: Option<&str>) -> Result<Vec<GridSchema>, ClientError> {
        fetch_entities_list::<GridSchema>(&self.url, format!("{}", SCHEMA_ROUTE), service_id)
    }
}
