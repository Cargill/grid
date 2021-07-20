use serde::Deserialize;

use super::{schema::DataType, Client};
use crate::error::ClientError;

#[derive(Debug, Deserialize)]
pub struct Location {
    pub location_id: String,
    pub location_namespace: String,
    pub owner: String,
    pub properties: Vec<LocationPropertyValue>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LocationPropertyValue {
    pub name: String,
    pub data_type: DataType,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    pub bytes_value: Option<Vec<u8>>,
    pub boolean_value: Option<bool>,
    pub number_value: Option<i64>,
    pub string_value: Option<String>,
    pub enum_value: Option<i32>,
    pub struct_values: Option<Vec<String>>,
    pub lat_long_value: Option<LatLong>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct LatLong {
    pub latitude: i64,
    pub longitude: i64,
}

pub trait LocationClient: Client {
    /// Fetches an agent based on its identified
    ///
    /// # Arguments
    ///
    /// * `id` - the location's identifier
    /// * `service_id` - optional - the service id to fetch the location from
    fn get_location(&self, id: String, service_id: Option<&str>) -> Result<Location, ClientError>;

    /// Fetches locations.
    ///
    /// # Arguments
    ///
    /// * `service_id` - optional - the service id to fetch locations from
    fn list_locations(&self, service_id: Option<&str>) -> Result<Vec<Location>, ClientError>;
}
