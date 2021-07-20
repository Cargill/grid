use super::fetch_methods::{fetch_entities_list, fetch_entity};
use super::ReqwestClient;
use crate::client::location::{Location, LocationClient};
use crate::error::ClientError;

const LOCATION_ROUTE: &str = "location";

impl LocationClient for ReqwestClient {
    /// Fetches an agent based on its identified
    ///
    /// # Arguments
    ///
    /// * `id` - the location's identifier
    /// * `service_id` - optional - the service id to fetch the location from
    fn get_location(&self, id: String, service_id: Option<&str>) -> Result<Location, ClientError> {
        fetch_entity::<Location>(&self.url, format!("{}/{}", LOCATION_ROUTE, id), service_id)
    }

    /// Fetches locations.
    ///
    /// # Arguments
    ///
    /// * `service_id` - optional - the service id to fetch locations from
    fn list_locations(&self, service_id: Option<&str>) -> Result<Vec<Location>, ClientError> {
        fetch_entities_list::<Location>(&self.url, format!("{}", LOCATION_ROUTE), service_id)
    }
}
