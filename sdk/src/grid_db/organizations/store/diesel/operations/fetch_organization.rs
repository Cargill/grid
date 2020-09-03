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

use super::OrganizationStoreOperations;
use crate::grid_db::error::StoreError;
use crate::grid_db::organizations::store::diesel::models::OrganizationModel;
use crate::grid_db::organizations::store::diesel::schema::organization;
use crate::grid_db::organizations::store::Organization;
use diesel::{prelude::*, result::Error::NotFound};

pub(in crate::grid_db::organizations) trait OrganizationStoreFetchOrganizationOperation {
    fn fetch_organization(
        &self,
        org_id: &str,
        service_id: Option<String>,
    ) -> Result<Option<Organization>, StoreError>;
}

#[cfg(feature = "postgres")]
impl<'a> OrganizationStoreFetchOrganizationOperation
    for OrganizationStoreOperations<'a, diesel::pg::PgConnection>
{
    fn fetch_organization(
        &self,
        org_id: &str,
        service_id: Option<String>,
    ) -> Result<Option<Organization>, StoreError> {
        let org = organization::table
            .filter(organization::service_id.eq(service_id))
            .filter(organization::org_id.eq(org_id))
            .first::<OrganizationModel>(self.conn)
            .map(Some)
            .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
            .map_err(|err| StoreError::QueryError {
                context: "Failed to fetch organization for org_id".to_string(),
                source: Box::new(err),
            })?
            .ok_or_else(|| {
                StoreError::NotFoundError(format!("Failed to find organization: {}", org_id,))
            })?;

        Ok(Some(Organization::from(org)))
    }
}
