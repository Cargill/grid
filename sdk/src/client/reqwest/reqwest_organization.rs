use super::fetch_methods::{fetch_entities_list, fetch_entity};
use super::ReqwestClient;
use crate::{
    client::organization::{Organization, OrganizationClient},
    error::ClientError,
};

const ORGANIZATION_ROUTE: &str = "organization";

impl OrganizationClient for ReqwestClient {
    /// Fetches organization by id and service_id
    ///
    /// # Arguments
    ///
    /// * `id` - the Organization Id
    /// * `service_id` - identifies the service to which the organization belongs
    fn get_organization(
        &self,
        id: String,
        service_id: Option<&str>,
    ) -> Result<Organization, ClientError> {
        fetch_entity::<Organization>(
            &self.url,
            format!("{}/{}", ORGANIZATION_ROUTE, id),
            service_id,
        )
    }

    /// Fetches all organizations that belong to the service_id
    ///
    /// # Arguments
    ///
    /// * `service_id` - identifies the service from which to filter organizations
    fn list_organizations(
        &self,
        service_id: Option<&str>,
    ) -> Result<Vec<Organization>, ClientError> {
        fetch_entities_list::<Organization>(
            &self.url,
            format!("{}", ORGANIZATION_ROUTE),
            service_id,
        )
    }
}
