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

#[cfg(feature = "diesel")]
pub mod diesel;
mod error;

pub use error::LocationStoreError;

#[cfg(feature = "diesel")]
use crate::grid_db::commits::MAX_COMMIT_NUM;
#[cfg(feature = "diesel")]
use crate::grid_db::locations::store::diesel::create_lat_long_value;

#[cfg(feature = "diesel")]
use self::diesel::models::{LocationAttributeModel, NewLocationAttributeModel, NewLocationModel};

/// Represents a Grid Location
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Location {
    pub location_id: String,
    pub location_namespace: String,
    pub owner: String,
    pub attributes: Vec<LocationAttribute>,
    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub service_id: Option<String>,
}

/// Represents a Grid Location Attribute
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct LocationAttribute {
    pub location_id: String,
    pub location_address: String,
    pub property_name: String,
    pub parent_property_name: Option<String>,
    pub data_type: String,
    pub bytes_value: Option<Vec<u8>>,
    pub boolean_value: Option<bool>,
    pub number_value: Option<i64>,
    pub string_value: Option<String>,
    pub enum_value: Option<i32>,
    pub struct_values: Option<Vec<LocationAttribute>>,
    pub lat_long_value: Option<LatLongValue>,
    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub service_id: Option<String>,
}

#[cfg(feature = "diesel")]
impl LocationAttribute {
    pub fn from_model(model: LocationAttributeModel) -> LocationAttribute {
        LocationAttribute {
            location_id: model.location_id,
            location_address: model.location_address,
            property_name: model.property_name,
            parent_property_name: model.parent_property_name,
            data_type: model.data_type,
            bytes_value: model.bytes_value,
            boolean_value: model.boolean_value,
            number_value: model.number_value,
            string_value: model.string_value,
            enum_value: model.enum_value,
            struct_values: None,
            lat_long_value: create_lat_long_value(model.latitude_value, model.longitude_value),
            start_commit_num: model.start_commit_num,
            end_commit_num: model.end_commit_num,
            service_id: model.service_id,
        }
    }

    pub fn from_model_with_children(
        model: LocationAttributeModel,
        children: Vec<LocationAttribute>,
    ) -> LocationAttribute {
        LocationAttribute {
            location_id: model.location_id,
            location_address: model.location_address,
            property_name: model.property_name,
            parent_property_name: model.parent_property_name,
            data_type: model.data_type,
            bytes_value: model.bytes_value,
            boolean_value: model.boolean_value,
            number_value: model.number_value,
            string_value: model.string_value,
            enum_value: model.enum_value,
            struct_values: Some(children),
            lat_long_value: create_lat_long_value(model.latitude_value, model.longitude_value),
            start_commit_num: model.start_commit_num,
            end_commit_num: model.end_commit_num,
            service_id: model.service_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct LatLong;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct LatLongValue(pub i64, pub i64);

pub trait LocationStore: Send + Sync {
    /// Adds a location to the underlying storage
    ///
    /// # Arguments
    ///
    ///  * `location` - The location to be added
    fn add_location(
        &self,
        location: Location,
        attributes: Vec<LocationAttribute>,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError>;

    /// Fetches a location from the underlying storage
    ///
    /// # Arguments
    ///
    ///  * `location_id` - The ID of the location to be fetched
    ///  * `service_id` - optional - The service ID to fetch the location from
    fn fetch_location(
        &self,
        location_id: &str,
        service_id: Option<String>,
    ) -> Result<Option<Location>, LocationStoreError>;

    /// Gets locations from the underlying storage
    ///
    /// # Arguments
    ///
    ///  * `service_id` - optional - The service ID to get the locations for
    fn list_locations(
        &self,
        service_id: Option<String>,
    ) -> Result<Vec<Location>, LocationStoreError>;

    /// Gets locations from the underlying storage
    ///
    /// # Arguments
    ///
    ///  * `location` - The updated location
    fn update_location(
        &self,
        location: Location,
        attributes: Vec<LocationAttribute>,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError>;
}

#[cfg(feature = "diesel")]
impl Into<NewLocationModel> for Location {
    fn into(self) -> NewLocationModel {
        NewLocationModel {
            location_id: self.location_id,
            location_namespace: self.location_namespace,
            owner: self.owner,
            start_commit_num: self.start_commit_num,
            end_commit_num: MAX_COMMIT_NUM,
            service_id: self.service_id,
        }
    }
}

#[cfg(feature = "diesel")]
pub fn make_location_attribute_models(
    attributes: &[LocationAttribute],
    parent_property_name: Option<String>,
) -> Vec<NewLocationAttributeModel> {
    let mut attrs = Vec::new();

    for attr in attributes {
        attrs.push(NewLocationAttributeModel {
            location_id: attr.location_id.to_string(),
            location_address: attr.location_address.to_string(),
            property_name: attr.property_name.to_string(),
            parent_property_name: parent_property_name.clone(),
            data_type: attr.data_type.to_string(),
            bytes_value: attr.bytes_value.clone(),
            boolean_value: attr.boolean_value,
            number_value: attr.number_value,
            string_value: attr.string_value.clone(),
            enum_value: attr.enum_value,
            latitude_value: attr.lat_long_value.clone().map(|lat_long| lat_long.0),
            longitude_value: attr.lat_long_value.clone().map(|lat_long| lat_long.1),
            start_commit_num: attr.start_commit_num,
            end_commit_num: MAX_COMMIT_NUM,
            service_id: attr.service_id.clone(),
        });

        if attr.struct_values.is_some() {
            let vals = attr.struct_values.as_ref().unwrap();
            if !vals.is_empty() {
                attrs.append(&mut make_location_attribute_models(
                    &vals,
                    Some(attr.property_name.to_string()),
                ));
            }
        }
    }

    attrs
}

impl<LS> LocationStore for Box<LS>
where
    LS: LocationStore + ?Sized,
{
    fn add_location(
        &self,
        location: Location,
        attributes: Vec<LocationAttribute>,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        (**self).add_location(location, attributes, current_commit_num)
    }

    fn fetch_location(
        &self,
        location_id: &str,
        service_id: Option<String>,
    ) -> Result<Option<Location>, LocationStoreError> {
        (**self).fetch_location(location_id, service_id)
    }

    fn list_locations(
        &self,
        service_id: Option<String>,
    ) -> Result<Vec<Location>, LocationStoreError> {
        (**self).list_locations(service_id)
    }

    fn update_location(
        &self,
        location: Location,
        attributes: Vec<LocationAttribute>,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        (**self).update_location(location, attributes, current_commit_num)
    }
}
