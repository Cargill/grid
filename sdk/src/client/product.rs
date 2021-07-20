use serde::Deserialize;

use crate::error::ClientError;

use super::Client;

#[derive(Debug, Deserialize)]
pub struct GridProduct {
    pub product_id: String,
    pub product_namespace: String,
    pub owner: String,
    pub properties: Vec<GridPropertyValue>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GridPropertyValue {
    pub name: String,
    pub data_type: String,
    pub bytes_value: Option<Vec<u8>>,
    pub boolean_value: Option<bool>,
    pub number_value: Option<i64>,
    pub string_value: Option<String>,
    pub enum_value: Option<u32>,
    pub struct_values: Option<Vec<String>>,
    pub lat_long_value: Option<LatLong>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LatLong {
    latitude: i64,
    longitude: i64,
}

pub trait ProductClient: Client {
    /// Fetches single product by identifier
    ///
    /// # Arguments
    ///
    /// * `product_id`: the product's identifier
    /// * `service_id`: the service id to fetch the product from
    fn get_product(
        &self,
        product_id: String,
        service_id: Option<&str>,
    ) -> Result<GridProduct, ClientError>;

    /// Fetches all products for a service
    ///
    /// # Arguments
    ///
    /// * `service_id`: the service id to fetch the products from
    fn list_products(&self, service_id: Option<&str>) -> Result<Vec<GridProduct>, ClientError>;
}
