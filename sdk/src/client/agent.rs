use crate::error::ClientError;
use serde::Deserialize;
use std::collections::HashMap;

use super::Client;

#[derive(Debug, Deserialize)]
pub struct Agent {
    pub public_key: String,
    pub org_id: String,
    pub active: bool,
    pub roles: Vec<String>,
    pub service_id: Option<String>,
    pub metadata: HashMap<String, String>,
}

pub trait AgentClient: Client {
    /// Fetches an agent based on its identifier
    ///
    /// # Arguments
    ///
    /// * `id` - the agent identifier, also known as public key
    /// * `service_id` - optional - the service id to fetch the agent from
    fn get_agent(&self, id: String, service_id: Option<&str>) -> Result<Agent, ClientError>;

    /// Fetches agents.
    ///
    /// # Arguments
    ///
    /// * `service_id` - optional - the service id to fetch the agenst from
    fn list_agents(&self, service_id: Option<&str>) -> Result<Vec<Agent>, ClientError>;
}
