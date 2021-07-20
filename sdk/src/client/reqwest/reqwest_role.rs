use super::fetch_methods::{fetch_entities_list, fetch_entity};
use super::ReqwestClient;
use crate::client::role::{GridRole, RoleClient};
use crate::error::ClientError;

const ROLES_ROUTE: &str = "role";

impl RoleClient for ReqwestClient {
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
    ) -> Result<GridRole, ClientError> {
        fetch_entity::<GridRole>(
            &self.url,
            format!("{}/{}/{}", ROLES_ROUTE, org_id, name),
            service_id,
        )
    }

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
    ) -> Result<Vec<GridRole>, ClientError> {
        fetch_entities_list::<GridRole>(
            &self.url,
            format!("{}/{}", ROLES_ROUTE, org_id),
            service_id,
        )
    }
}
