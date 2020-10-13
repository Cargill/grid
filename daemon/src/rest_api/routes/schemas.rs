// Copyright 2019 Cargill Incorporated
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

use crate::rest_api::{
    error::RestApiResponseError, routes::DbExecutor, AcceptServiceIdParam, AppState, QueryServiceId,
};

use actix::{Handler, Message, SyncContext};
use actix_web::{web, HttpResponse};
use grid_sdk::grid_db::schemas::store::{PropertyDefinition, Schema};
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GridSchemaSlice {
    pub name: String,
    pub description: String,
    pub owner: String,
    pub properties: Vec<GridPropertyDefinitionSlice>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

impl GridSchemaSlice {
    pub fn from_schema(schema: &GridSchema, properties: Vec<GridPropertyDefinitionSlice>) -> Self {
        Self {
            name: schema.name.clone(),
            description: schema.description.clone(),
            owner: schema.owner.clone(),
            properties,
            service_id: schema.service_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridPropertyDefinitionSlice {
    pub name: String,
    pub schema_name: String,
    pub data_type: String,
    pub required: bool,
    pub description: String,
    pub number_exponent: i64,
    pub enum_options: Vec<String>,
    pub struct_properties: Vec<GridPropertyDefinitionSlice>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

impl GridPropertyDefinitionSlice {
    pub fn from_definition(definition: &GridPropertyDefinition) -> Self {
        Self {
            name: definition.name.clone(),
            schema_name: definition.schema_name.clone(),
            data_type: definition.data_type.clone(),
            required: definition.required,
            description: definition.description.clone(),
            number_exponent: definition.number_exponent,
            enum_options: definition.enum_options.clone(),
            struct_properties: vec![],
            service_id: definition.service_id.clone(),
        }
    }

    pub fn from_slices(
        definition: &GridPropertyDefinition,
        slices: Vec<GridPropertyDefinitionSlice>,
    ) -> Self {
        Self {
            name: definition.name.clone(),
            schema_name: definition.schema_name.clone(),
            data_type: definition.data_type.clone(),
            required: definition.required,
            description: definition.description.clone(),
            number_exponent: definition.number_exponent,
            enum_options: definition.enum_options.clone(),
            struct_properties: slices,
            service_id: definition.service_id.clone(),
        }
    }
}

struct ListGridSchemas {
    service_id: Option<String>,
}

impl Message for ListGridSchemas {
    type Result = Result<Vec<GridSchemaSlice>, RestApiResponseError>;
}

impl Handler<ListGridSchemas> for DbExecutor {
    type Result = Result<Vec<GridSchemaSlice>, RestApiResponseError>;

    fn handle(&mut self, msg: ListGridSchemas, _: &mut SyncContext<Self>) -> Self::Result {
        let conn = &*self.connection_pool.get()?;
        let mut properties = db::list_grid_property_definitions(conn, msg.service_id.as_deref())?
            .into_iter()
            .fold(HashMap::new(), |mut acc, definition| {
                acc.entry(definition.schema_name.to_string())
                    .or_insert_with(Vec::new)
                    .push(definition);
                acc
            });

        let fetched_schemas =
            db::list_grid_schemas(&*self.connection_pool.get()?, msg.service_id.as_deref())?
                .iter()
                .map(|schema| {
                    let properties = properties.remove(&schema.name).unwrap_or_else(Vec::new);
                    let definitions = make_definition_slices(
                        conn,
                        &schema.name,
                        &properties,
                        msg.service_id.as_deref(),
                    )?;
                    Ok(GridSchemaSlice::from_schema(schema, definitions))
                })
                .collect::<Result<Vec<GridSchemaSlice>, RestApiResponseError>>()?;

        Ok(fetched_schemas)
    }
}

pub async fn list_grid_schemas(
    state: web::Data<AppState>,
    query: web::Query<QueryServiceId>,
    _: AcceptServiceIdParam,
) -> Result<HttpResponse, RestApiResponseError> {
    state
        .database_connection
        .send(ListGridSchemas {
            service_id: query.into_inner().service_id,
        })
        .await?
        .map(|schemas| HttpResponse::Ok().json(schemas))
}

struct FetchGridSchema {
    name: String,
    service_id: Option<String>,
}

impl Message for FetchGridSchema {
    type Result = Result<GridSchemaSlice, RestApiResponseError>;
}

impl Handler<FetchGridSchema> for DbExecutor {
    type Result = Result<GridSchemaSlice, RestApiResponseError>;

    fn handle(&mut self, msg: FetchGridSchema, _: &mut SyncContext<Self>) -> Self::Result {
        let properties = db::list_grid_property_definitions_with_schema_name(
            &*self.connection_pool.get()?,
            &msg.name,
            msg.service_id.as_deref(),
        )?;

        let definitions = make_definition_slices(
            &*self.connection_pool.get()?,
            &msg.name,
            &properties,
            msg.service_id.as_deref(),
        )?;

        let fetched_schema = match db::fetch_grid_schema(
            &*self.connection_pool.get()?,
            &msg.name,
            msg.service_id.as_deref(),
        )? {
            Some(schema) => GridSchemaSlice::from_schema(&schema, definitions),
            None => {
                return Err(RestApiResponseError::NotFoundError(format!(
                    "Could not find schema with name: {}",
                    msg.name
                )));
            }
        };

        Ok(fetched_schema)
    }
}

pub async fn fetch_grid_schema(
    state: web::Data<AppState>,
    schema_name: web::Path<String>,
    query: web::Query<QueryServiceId>,
    _: AcceptServiceIdParam,
) -> Result<HttpResponse, RestApiResponseError> {
    state
        .database_connection
        .send(FetchGridSchema {
            name: schema_name.into_inner(),
            service_id: query.into_inner().service_id,
        })
        .await?
        .map(|schema| HttpResponse::Ok().json(schema))
}

fn make_definition_slices(
    conn: &PgConnection,
    schema_name: &str,
    definitions: &[GridPropertyDefinition],
    service_id: Option<&str>,
) -> Result<Vec<GridPropertyDefinitionSlice>, RestApiResponseError> {
    let mut slices = Vec::new();

    for def in definitions {
        make_definition_slices_aux(conn, schema_name, &def.name, service_id, &mut slices)?;
    }

    Ok(slices)
}

fn make_definition_slices_aux(
    conn: &PgConnection,
    schema_name: &str,
    name: &str,
    service_id: Option<&str>,
    slices: &mut Vec<GridPropertyDefinitionSlice>,
) -> Result<(), RestApiResponseError> {
    let definition = db::get_property_definition_by_name(conn, schema_name, name, service_id)?;

    if definition.struct_properties.is_empty() {
        slices.push(GridPropertyDefinitionSlice::from_definition(&definition));
    } else {
        let mut sub_slices = Vec::new();
        for def_name in definition.struct_properties.clone() {
            make_definition_slices_aux(conn, schema_name, &def_name, service_id, &mut sub_slices)?;
        }

        slices.push(GridPropertyDefinitionSlice::from_slices(
            &definition,
            sub_slices,
        ));
    }
    Ok(())
}
