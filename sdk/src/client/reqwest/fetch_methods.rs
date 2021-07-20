use crate::error::ClientError;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Paging {
    current: String,
    offset: i64,
    limit: i64,
    total: i64,
    first: String,
    prev: String,
    next: Option<String>,
    last: String,
}

#[derive(Debug, Deserialize)]
pub struct ListSlice<T> {
    pub data: Vec<T>,
    pub paging: Paging,
}

/// Fetches and serializes T entities from REST API
///
/// # Arguments
///
/// * `url` - The base url of the request
/// * `route` - the route to find the entity
/// * `service_id` - optional - the service id to fetch the entities from
pub fn fetch_entities_list<T: DeserializeOwned>(
    url: &String,
    route: String,
    service_id: Option<&str>,
) -> Result<Vec<T>, ClientError> {
    let client = Client::new();
    let mut final_url = format!("{}/{}", url, route);
    if let Some(service_id) = service_id {
        final_url = format!("{}?service_id={}", final_url, service_id);
    }

    let mut entities: Vec<T> = Vec::new();

    loop {
        let response = client.get(&final_url).send()?;

        if !response.status().is_success() {
            return Err(ClientError::DaemonError(response.text()?));
        }

        let mut entity_list_slice = response.json::<ListSlice<T>>()?;

        entities.append(&mut entity_list_slice.data);

        if let Some(next) = entity_list_slice.paging.next {
            final_url = format!("{}{}", url, next);
        } else {
            break;
        }
    }

    Ok(entities)
}

/// Fetches and serializes single T Entity from REST API
///
/// # Arguments
///
/// * `url` - the base url of the request
/// * `route` - the identifying route where to find the entity
/// * `service_id` - optional - the service id to fetch the entity from
pub fn fetch_entity<T: DeserializeOwned>(
    url: &String,
    route: String,
    service_id: Option<&str>,
) -> Result<T, ClientError> {
    let client = Client::new();
    let mut final_url = format!("{}/{}", url, route);
    if let Some(service_id) = service_id {
        final_url = format!("{}?service_id={}", final_url, service_id);
    }

    let response = client.get(&final_url).send()?;

    if !response.status().is_success() {
        return Err(ClientError::DaemonError(response.text()?));
    }

    let agent = response.json::<T>()?;

    Ok(agent)
}
