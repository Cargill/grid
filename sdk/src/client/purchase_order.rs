use std::time::SystemTime;

use serde::Deserialize;

use crate::error::ClientError;

use super::Client;

#[cfg(feature = "purchase-order")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseOrder {
    org_id: String,
    uuid: String,
    workflow_status: String,
    is_closed: bool,
    accepted_version_id: Option<String>,
    versions: Vec<PurchaseOrderVersion>,
    created_at: SystemTime,
}

#[cfg(feature = "purchase-order")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseOrderVersion {
    version_id: String,
    workflow_status: String,
    is_draft: bool,
    current_revision_id: u64,
    revisions: Vec<PurchaseOrderRevision>,
}

#[cfg(feature = "purchase-order")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseOrderRevision {
    revision_id: u64,
    order_xml_v3_4: String,
    submitter: String,
    created_at: u64,
}

#[cfg(feature = "purchase-order")]
pub trait PurchaseOrderClient: Client {
    /// Retrieves the purchase order with the specified `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The uuid of the `PurchaseOrder` to be retrieved
    #[cfg(feature = "purchase-order")]
    fn get_purchase_order(&self, id: String) -> Result<Option<PurchaseOrder>, ClientError>;

    /// Retrieves the purchase order version with the given `version_id` of the purchase
    /// order with the given `id`
    ///
    /// # Arguments
    ///
    /// * `id` - The uuid of the `PurchaseOrder` containing the `PurchaseOrderVersion` to be retrieved
    /// * `version_id` - The version id of the `PurchaseOrderVersion` to be retrieved
    #[cfg(feature = "purchase-order")]
    fn get_purchase_order_version(
        &self,
        id: String,
        version_id: String,
    ) -> Result<Option<PurchaseOrderVersion>, ClientError>;

    /// Retrieves the purchase order revision with the given `revision_id` of
    /// the purchase order version with the given `version_id` of the purchase order with the given `id`
    ///
    /// # Arguments
    ///
    /// * `id` - The uuid of the `PurchaseOrder` containing the `PurchaseOrderRevision` to be retrieved
    /// * `version_id` - The version id of the `PurchaseOrderVersion` containing the
    ///   `PurchaseOrderRevision` to be retrieved
    /// * `revision_id` - The revision number of the `PurchaseOrderRevision` to be retrieved
    #[cfg(feature = "purchase-order")]
    fn get_purchase_order_revision(
        &self,
        id: String,
        version_id: String,
        revision_id: String,
    ) -> Result<Option<PurchaseOrderRevision>, ClientError>;

    /// lists purchase orders.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter to apply to the list of `PurchaseOrder`s
    #[cfg(feature = "purchase-order")]
    fn list_purchase_orders(&self, filter: Option<&str>)
        -> Result<Vec<PurchaseOrder>, ClientError>;

    /// lists the purchase order versions of a specific purchase order.
    ///
    /// # Arguments
    ///
    /// * `id` - The uuid of the `PurchaseOrder` containing the `PurchaseOrderVersion`s to be listed
    /// * `filter` - Filter to apply to the list of purchase orders
    #[cfg(feature = "purchase-order")]
    fn list_purchase_order_versions(
        &self,
        id: String,
        filter: Option<&str>,
    ) -> Result<Vec<PurchaseOrderVersion>, ClientError>;

    /// lists the purchase order revisions of a specific purchase order version.
    ///
    /// # Arguments
    ///
    /// * `id` - The uuid of the `PurchaseOrder` containing the `PurchaseOrderRevision`s to be listed
    /// * `version_id` - The version id of the `PurchaseOrderVersion` containing
    ///   the `PurchaseOrderRevision`s to be listed
    #[cfg(feature = "purchase-order")]
    fn list_purchase_order_revisions(
        &self,
        id: String,
        version_id: String,
        filter: Option<&str>,
    ) -> Result<Vec<PurchaseOrderRevision>, ClientError>;
}
