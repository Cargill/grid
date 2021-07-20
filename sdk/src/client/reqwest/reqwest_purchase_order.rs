#[cfg(feature = "purchase-order")]
use crate::client::purchase_order::{
    PurchaseOrder, PurchaseOrderClient, PurchaseOrderRevision, PurchaseOrderVersion,
};
use crate::error::ClientError;

use super::ReqwestClient;

#[cfg(feature = "purchase-order")]
impl PurchaseOrderClient for ReqwestClient {
    /// Retrieves the purchase order with the specified `id`.
    fn get_purchase_order(&self, _id: String) -> Result<Option<PurchaseOrder>, ClientError> {
        unimplemented!()
    }

    /// Retrieves the purchase order version with the given `version_id` of the purchase order
    /// with the given `id`
    fn get_purchase_order_version(
        &self,
        _id: String,
        _version_id: String,
    ) -> Result<Option<PurchaseOrderVersion>, ClientError> {
        unimplemented!()
    }

    /// Retrieves the purchase order revision with the given `revision_id` of the purchase
    /// order version with the given `version_id` of the purchase order with the given `id`
    fn get_purchase_order_revision(
        &self,
        _id: String,
        _version_id: String,
        _revision_id: String,
    ) -> Result<Option<PurchaseOrderRevision>, ClientError> {
        unimplemented!()
    }

    /// lists purchase orders.
    fn list_purchase_orders(
        &self,
        _filter: Option<&str>,
    ) -> Result<Vec<PurchaseOrder>, ClientError> {
        unimplemented!()
    }

    /// lists the purchase order versions of a specific purchase order.
    fn list_purchase_order_versions(
        &self,
        _id: String,
        _filter: Option<&str>,
    ) -> Result<Vec<PurchaseOrderVersion>, ClientError> {
        unimplemented!()
    }

    /// lists the purchase order revisions of a specific purchase order version.
    fn list_purchase_order_revisions(
        &self,
        _id: String,
        _version_id: String,
        _filter: Option<&str>,
    ) -> Result<Vec<PurchaseOrderRevision>, ClientError> {
        unimplemented!()
    }
}
