use serde::Deserialize;

use crate::error::ClientError;

use super::Client;

#[derive(Debug, Deserialize)]
pub struct AlternateId {
    pub id_type: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct OrganizationMetadata {
    pub key: String,
    pub value: String,
    pub service_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Organization {
    pub org_id: String,
    pub name: String,
    pub locations: Vec<String>,
    pub alternate_ids: Vec<AlternateId>,
    pub metadata: Vec<OrganizationMetadata>,
    pub service_id: Option<String>,
}

pub trait OrganizationClient: Client {
    /// Fetches organization by id
    ///
    /// # Arguments
    ///
    /// * `id` - the Organization Id
    /// * `service_id` - identifies the service to fetch the organization from
    fn get_organization(
        &self,
        id: String,
        service_id: Option<&str>,
    ) -> Result<Organization, ClientError>;

    /// Fetches all organizations
    ///
    /// # Arguments
    ///
    /// * `service_id` - identifies the service to fetch the organizations from
    fn list_organizations(
        &self,
        service_id: Option<&str>,
    ) -> Result<Vec<Organization>, ClientError>;
}
