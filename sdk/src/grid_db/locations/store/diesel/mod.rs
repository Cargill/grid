// Copyright 2018-2020 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod models;
mod operations;
pub(in crate::grid_db) mod schema;

use diesel::r2d2::{ConnectionManager, Pool};

use crate::grid_db::locations::store::make_location_attribute_models;

use super::diesel::models::{LocationModel, NewLocationModel};
use super::{LatLongValue, Location, LocationAttribute, LocationStore, LocationStoreError};
use crate::database::DatabaseError;
use operations::add_location::LocationStoreAddLocationOperation as _;
use operations::fetch_location::LocationStoreFetchLocationOperation as _;
use operations::list_locations::LocationStoreListLocationsOperation as _;
use operations::update_location::LocationStoreUpdateLocationOperation as _;
use operations::LocationStoreOperations;

/// Manages creating organizations in the database
#[derive(Clone)]
pub struct DieselLocationStore<C: diesel::Connection + 'static> {
    connection_pool: Pool<ConnectionManager<C>>,
}

#[cfg(feature = "diesel")]
impl<C: diesel::Connection> DieselLocationStore<C> {
    /// Creates a new DieselLocationStore
    ///
    /// # Arguments
    ///
    ///  * `connection_pool`: connection pool to the database
    // Allow dead code if diesel feature is not enabled
    #[allow(dead_code)]
    pub fn new(connection_pool: Pool<ConnectionManager<C>>) -> Self {
        DieselLocationStore { connection_pool }
    }
}

#[cfg(feature = "postgres")]
impl LocationStore for DieselLocationStore<diesel::pg::PgConnection> {
    fn add_location(
        &self,
        location: Location,
        attributes: Vec<LocationAttribute>,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            DatabaseError::ConnectionError {
                context: "Could not get connection pool".to_string(),
                source: Box::new(err),
            }
        })?)
        .add_location(
            location.into(),
            make_location_attribute_models(&attributes, None),
            current_commit_num,
        )
    }

    fn fetch_location(
        &self,
        location_id: &str,
        service_id: Option<String>,
    ) -> Result<Option<Location>, LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            DatabaseError::ConnectionError {
                context: "Could not get connection pool".to_string(),
                source: Box::new(err),
            }
        })?)
        .fetch_location(location_id, service_id)
    }

    fn list_locations(
        &self,
        service_id: Option<String>,
    ) -> Result<Vec<Location>, LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            DatabaseError::ConnectionError {
                context: "Could not get connection pool".to_string(),
                source: Box::new(err),
            }
        })?)
        .list_locations(service_id)
    }

    fn update_location(
        &self,
        location: Location,
        attributes: Vec<LocationAttribute>,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            DatabaseError::ConnectionError {
                context: "Could not get connection pool".to_string(),
                source: Box::new(err),
            }
        })?)
        .update_location(
            location.into(),
            make_location_attribute_models(&attributes, None),
            current_commit_num,
        )
    }
}

impl From<(i64, i64)> for LatLongValue {
    fn from((lat, long): (i64, i64)) -> Self {
        Self(lat, long)
    }
}

impl From<(LocationModel, Vec<LocationAttribute>)> for Location {
    fn from((location, attributes): (LocationModel, Vec<LocationAttribute>)) -> Self {
        Self {
            location_id: location.location_id,
            location_namespace: location.location_namespace,
            owner: location.owner,
            attributes,
            start_commit_num: location.start_commit_num,
            end_commit_num: location.end_commit_num,
            service_id: location.service_id,
        }
    }
}

impl From<LocationModel> for Location {
    fn from(location: LocationModel) -> Self {
        Self {
            location_id: location.location_id,
            location_namespace: location.location_namespace,
            owner: location.owner,
            attributes: Vec::new(),
            start_commit_num: location.start_commit_num,
            end_commit_num: location.end_commit_num,
            service_id: location.service_id,
        }
    }
}

impl From<NewLocationModel> for Location {
    fn from(location: NewLocationModel) -> Self {
        Self {
            location_id: location.location_id,
            location_namespace: location.location_namespace,
            owner: location.owner,
            attributes: Vec::new(),
            start_commit_num: location.start_commit_num,
            end_commit_num: location.end_commit_num,
            service_id: location.service_id,
        }
    }
}

pub fn create_lat_long_value(lat: Option<i64>, long: Option<i64>) -> Option<LatLongValue> {
    if let Some(latitude) = lat {
        if let Some(longitude) = long {
            Some(LatLongValue::from((latitude, longitude)))
        } else {
            None
        }
    } else {
        None
    }
}
