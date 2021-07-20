use serde::Deserialize;

use crate::error::ClientError;

use super::Client;

#[derive(Debug, Deserialize)]
pub struct GridRole {
    pub org_id: String,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub permissions: Vec<String>,
    pub inherit_from: Vec<GridInheritFrom>,
    pub allowed_organizations: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GridInheritFrom {
    pub role_name: String,
    pub org_id: String,
}

pub trait RoleClient: Client {
    /// Fetches a single role from an organization based on name
    ///
    /// # Arguments
    ///
    /// * `org_id` - identifier of the role's organization
    /// * `name` - the name of the role (identifier)
    /// * `service_id` - optional - the service id to fetch the role from
    fn get_role(
        &self,
        org_id: String,
        name: String,
        service_id: Option<&str>,
    ) -> Result<GridRole, ClientError>;

    /// Fetches a list of roles for the organization
    ///
    /// # Arguments
    ///
    /// * `org_id` - identifier of the role's organization
    /// * `service_id` - optional - the service id to fetch the roles from
    fn list_roles(
        &self,
        org_id: String,
        service_id: Option<&str>,
    ) -> Result<Vec<GridRole>, ClientError>;
}
