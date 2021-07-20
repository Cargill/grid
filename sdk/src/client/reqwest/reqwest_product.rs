use super::{
    fetch_methods::{fetch_entities_list, fetch_entity},
    ReqwestClient,
};
use crate::{
    client::product::{GridProduct, ProductClient},
    error::ClientError,
};

const PRODUCT_ROUTE: &str = "product";

impl ProductClient for ReqwestClient {
    /// Fetches single product by identifier
    ///
    /// # Arguments
    ///
    /// * `product_id`: the product's identifier
    /// * `service_id`: the service id to fetch the product from
    fn get_product(
        &self,
        id: String,
        service_id: Option<&str>,
    ) -> Result<GridProduct, ClientError> {
        fetch_entity::<GridProduct>(&self.url, format!("{}/{}", PRODUCT_ROUTE, id), service_id)
    }

    /// Fetches all products for a service
    ///
    /// # Arguments
    ///
    /// * `service_id`: the service id to fetch the products from
    fn list_products(&self, service_id: Option<&str>) -> Result<Vec<GridProduct>, ClientError> {
        fetch_entities_list::<GridProduct>(&self.url, format!("{}", PRODUCT_ROUTE), service_id)
    }
}
