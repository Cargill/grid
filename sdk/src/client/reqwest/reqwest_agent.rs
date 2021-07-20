use super::fetch_methods::{fetch_entities_list, fetch_entity};
use super::ReqwestClient;
use crate::client::agent::{Agent, AgentClient};
use crate::error::ClientError;

// pub struct ReqwestAgentClient {
//     url: String
// }

// impl ReqwestAgentClient {
//     pub fn new(url: String) -> Self {
//         ReqwestSchemaClient { url }
//     }
// }

const AGENT_ROUTE: &str = "agent";

impl AgentClient for ReqwestClient {
    /// Fetches an agent based on its identified
    ///
    /// # Arguments
    ///
    /// * `id` - the agent identifier, also known as public key
    /// * `service_id` - optional - the service id to fetch the agent from
    fn get_agent(&self, id: String, service_id: Option<&str>) -> Result<Agent, ClientError> {
        fetch_entity::<Agent>(&self.url, format!("{}/{}", AGENT_ROUTE, id), service_id)
    }

    /// Fetches agents.
    ///
    /// # Arguments
    ///
    /// * `service_id` - optional - the service id to fetch the agenst from
    fn list_agents(&self, service_id: Option<&str>) -> Result<Vec<Agent>, ClientError> {
        fetch_entities_list::<Agent>(&self.url, format!("{}", AGENT_ROUTE), service_id)
    }
}
