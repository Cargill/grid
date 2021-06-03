/*
 * Copyright 2020 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

use std::time::Duration;

#[cfg(feature = "location")]
use grid_sdk::locations::addressing::GRID_LOCATION_NAMESPACE;
#[cfg(feature = "pike")]
use grid_sdk::pike::addressing::PIKE_NAMESPACE;
#[cfg(feature = "product")]
use grid_sdk::products::addressing::GRID_PRODUCT_NAMESPACE;
#[cfg(feature = "purchase-order")]
use grid_sdk::purchase_order::addressing::GRID_PURCHASE_ORDER_NAMESPACE;
#[cfg(feature = "schema")]
use grid_sdk::schemas::addressing::GRID_SCHEMA_NAMESPACE;

use cylinder::{
    jwt::JsonWebTokenBuilder, secp256k1::Secp256k1Context, Context, PrivateKey, Signer,
};
use sabre_sdk::protocol::payload::{
    CreateContractActionBuilder, CreateContractRegistryActionBuilder,
    CreateNamespaceRegistryActionBuilder, CreateNamespaceRegistryPermissionActionBuilder,
};
use scabbard::client::{ReqwestScabbardClientBuilder, ScabbardClient, ServiceId};
use semver::Version;
#[cfg(any(
    feature = "location",
    feature = "pike",
    feature = "product",
    feature = "purchase-order",
    feature = "schema"
))]
use transact::contract::archive::default_scar_path;
use transact::{
    contract::archive::SmartContractArchive,
    protocol::{batch::BatchBuilder, transaction::Transaction},
};

use crate::splinter::app_auth_handler::error::AppAuthHandlerError;

const SCABBARD_SUBMISSION_WAIT_SECS: u64 = 10;

pub fn setup_grid(
    scabbard_admin_key: &str,
    proposed_admin_pubkeys: Vec<String>,
    splinterd_url: &str,
    service_id: &str,
    circuit_id: &str,
) -> Result<(), AppAuthHandlerError> {
    #[cfg(any(
        feature = "location",
        feature = "pike",
        feature = "product",
        feature = "purchase-order",
        feature = "schema"
    ))]
    let version = Version::parse(env!("CARGO_PKG_VERSION"))
        .map(|v| format!("^{}.{}, {}", v.major, v.minor, v))
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    let context = Secp256k1Context::new();
    let private_key = PrivateKey::new_from_hex(&scabbard_admin_key)
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;
    let signer = context.new_signer(private_key);

    // The node with the first key in the list of scabbard admins is responsible for setting up xo
    let public_key = signer
        .public_key()
        .map(|pk| pk.as_hex())
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    let is_submitter = match proposed_admin_pubkeys.get(0) {
        Some(submitting_key) => &public_key == submitting_key,
        None => false,
    };
    if !is_submitter {
        return Ok(());
    }

    let mut txns = Vec::new();

    // Make Pike transactions
    #[cfg(feature = "pike")]
    make_pike_txns(&mut txns, &version, &*signer)?;

    // Make Product transactions
    #[cfg(feature = "product")]
    make_product_txns(&mut txns, &version, &*signer)?;

    // Make Location transactions
    #[cfg(feature = "location")]
    make_location_txns(&mut txns, &version, &*signer)?;

    // Make schema transactions
    #[cfg(feature = "schema")]
    make_schema_txns(&mut txns, &version, &*signer)?;

    // Make Purchase Order transactions
    #[cfg(feature = "purchase-order")]
    make_purchase_order_txns(&mut txns, &version, &*signer)?;

    let batch = BatchBuilder::new()
        .with_transactions(txns)
        .build(&*signer)?;

    let encoded_token = JsonWebTokenBuilder::new()
        .build(&*signer)
        .map(|token| format!("Bearer Cylinder:{}", token))
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    let client = ReqwestScabbardClientBuilder::default()
        .with_url(splinterd_url)
        .with_auth(&encoded_token)
        .build()
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    client
        .submit(
            &ServiceId::new(circuit_id, service_id),
            vec![batch],
            Some(Duration::from_secs(SCABBARD_SUBMISSION_WAIT_SECS)),
        )
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    Ok(())
}

#[cfg(feature = "pike")]
fn make_pike_txns(
    txns: &mut Vec<Transaction>,
    version: &str,
    signer: &dyn Signer,
) -> Result<(), AppAuthHandlerError> {
    let pike_contract =
        SmartContractArchive::from_scar_file("grid-pike", &version, &default_scar_path())?;

    let public_key = signer
        .public_key()
        .map(|pk| pk.as_hex())
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    let pike_contract_registry_txn = CreateContractRegistryActionBuilder::new()
        .with_name(String::from(&pike_contract.metadata.name))
        .with_owners(vec![public_key.clone()])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let pike_contract_txn = make_upload_contract_txn(signer, &pike_contract, PIKE_NAMESPACE)?;
    let pike_namespace_registry_txn = CreateNamespaceRegistryActionBuilder::new()
        .with_namespace(PIKE_NAMESPACE.into())
        .with_owners(vec![public_key])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;

    let pike_namespace_permissions_txn = CreateNamespaceRegistryPermissionActionBuilder::new()
        .with_namespace(PIKE_NAMESPACE.into())
        .with_contract_name(pike_contract.metadata.name)
        .with_read(true)
        .with_write(true)
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;

    txns.append(&mut vec![
        pike_contract_registry_txn,
        pike_contract_txn,
        pike_namespace_registry_txn,
        pike_namespace_permissions_txn,
    ]);

    Ok(())
}

#[cfg(feature = "product")]
fn make_product_txns(
    txns: &mut Vec<Transaction>,
    version: &str,
    signer: &dyn Signer,
) -> Result<(), AppAuthHandlerError> {
    let product_contract =
        SmartContractArchive::from_scar_file("grid-product", &version, &default_scar_path())?;

    let public_key = signer
        .public_key()
        .map(|pk| pk.as_hex())
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    let product_contract_registry_txn = CreateContractRegistryActionBuilder::new()
        .with_name(String::from(&product_contract.metadata.name))
        .with_owners(vec![public_key.clone()])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let product_contract_txn =
        make_upload_contract_txn(signer, &product_contract, GRID_PRODUCT_NAMESPACE)?;
    let product_namespace_registry_txn = CreateNamespaceRegistryActionBuilder::new()
        .with_namespace(GRID_PRODUCT_NAMESPACE.into())
        .with_owners(vec![public_key])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let product_namespace_permissions_txn = CreateNamespaceRegistryPermissionActionBuilder::new()
        .with_namespace(GRID_PRODUCT_NAMESPACE.into())
        .with_contract_name(product_contract.metadata.name.clone())
        .with_read(true)
        .with_write(true)
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let product_pike_namespace_permissions_txn =
        CreateNamespaceRegistryPermissionActionBuilder::new()
            .with_namespace(PIKE_NAMESPACE.into())
            .with_contract_name(product_contract.metadata.name.clone())
            .with_read(true)
            .with_write(true)
            .into_payload_builder()?
            .into_transaction_builder(signer)?
            .build(signer)?;
    let product_schema_namespace_permissions_txn =
        CreateNamespaceRegistryPermissionActionBuilder::new()
            .with_namespace(GRID_SCHEMA_NAMESPACE.into())
            .with_contract_name(product_contract.metadata.name)
            .with_read(true)
            .with_write(true)
            .into_payload_builder()?
            .into_transaction_builder(signer)?
            .build(signer)?;

    txns.append(&mut vec![
        product_contract_registry_txn,
        product_contract_txn,
        product_namespace_registry_txn,
        product_namespace_permissions_txn,
        product_pike_namespace_permissions_txn,
        product_schema_namespace_permissions_txn,
    ]);

    Ok(())
}

#[cfg(feature = "location")]
fn make_location_txns(
    txns: &mut Vec<Transaction>,
    version: &str,
    signer: &dyn Signer,
) -> Result<(), AppAuthHandlerError> {
    let location_contract =
        SmartContractArchive::from_scar_file("grid-location", &version, &default_scar_path())?;

    let public_key = signer
        .public_key()
        .map(|pk| pk.as_hex())
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    let location_contract_registry_txn = CreateContractRegistryActionBuilder::new()
        .with_name(String::from(&location_contract.metadata.name))
        .with_owners(vec![public_key.clone()])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let location_contract_txn =
        make_upload_contract_txn(signer, &location_contract, GRID_LOCATION_NAMESPACE)?;
    let location_namespace_registry_txn = CreateNamespaceRegistryActionBuilder::new()
        .with_namespace(GRID_LOCATION_NAMESPACE.into())
        .with_owners(vec![public_key])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let location_namespace_permissions_txn = CreateNamespaceRegistryPermissionActionBuilder::new()
        .with_namespace(GRID_LOCATION_NAMESPACE.into())
        .with_contract_name(location_contract.metadata.name.clone())
        .with_read(true)
        .with_write(true)
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let location_pike_namespace_permissions_txn =
        CreateNamespaceRegistryPermissionActionBuilder::new()
            .with_namespace(PIKE_NAMESPACE.into())
            .with_contract_name(location_contract.metadata.name.clone())
            .with_read(true)
            .with_write(true)
            .into_payload_builder()?
            .into_transaction_builder(signer)?
            .build(signer)?;
    let location_schema_namespace_permissions_txn =
        CreateNamespaceRegistryPermissionActionBuilder::new()
            .with_namespace(GRID_SCHEMA_NAMESPACE.into())
            .with_contract_name(location_contract.metadata.name)
            .with_read(true)
            .with_write(true)
            .into_payload_builder()?
            .into_transaction_builder(signer)?
            .build(signer)?;

    txns.append(&mut vec![
        location_contract_registry_txn,
        location_contract_txn,
        location_namespace_registry_txn,
        location_namespace_permissions_txn,
        location_pike_namespace_permissions_txn,
        location_schema_namespace_permissions_txn,
    ]);

    Ok(())
}

#[cfg(feature = "schema")]
fn make_schema_txns(
    txns: &mut Vec<Transaction>,
    version: &str,
    signer: &dyn Signer,
) -> Result<(), AppAuthHandlerError> {
    let schema_contract =
        SmartContractArchive::from_scar_file("grid-schema", &version, &default_scar_path())?;

    let public_key = signer
        .public_key()
        .map(|pk| pk.as_hex())
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    let schema_contract_registry_txn = CreateContractRegistryActionBuilder::new()
        .with_name(String::from(&schema_contract.metadata.name))
        .with_owners(vec![public_key.clone()])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let schema_contract_txn =
        make_upload_contract_txn(signer, &schema_contract, GRID_SCHEMA_NAMESPACE)?;
    let schema_namespace_registry_txn = CreateNamespaceRegistryActionBuilder::new()
        .with_namespace(GRID_SCHEMA_NAMESPACE.into())
        .with_owners(vec![public_key])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let schema_namespace_permissions_txn = CreateNamespaceRegistryPermissionActionBuilder::new()
        .with_namespace(GRID_SCHEMA_NAMESPACE.into())
        .with_contract_name(schema_contract.metadata.name.clone())
        .with_read(true)
        .with_write(true)
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let schema_pike_namespace_permissions_txn =
        CreateNamespaceRegistryPermissionActionBuilder::new()
            .with_namespace(PIKE_NAMESPACE.into())
            .with_contract_name(schema_contract.metadata.name)
            .with_read(true)
            .with_write(true)
            .into_payload_builder()?
            .into_transaction_builder(signer)?
            .build(signer)?;

    txns.append(&mut vec![
        schema_contract_registry_txn,
        schema_contract_txn,
        schema_namespace_registry_txn,
        schema_namespace_permissions_txn,
        schema_pike_namespace_permissions_txn,
    ]);

    Ok(())
}

#[cfg(feature = "purchase-order")]
fn make_purchase_order_txns(
    txns: &mut Vec<Transaction>,
    version: &str,
    signer: &dyn Signer,
) -> Result<(), AppAuthHandlerError> {
    let purchase_order_contract = SmartContractArchive::from_scar_file(
        "grid-purchase-order",
        &version,
        &default_scar_path(),
    )?;

    let public_key = signer
        .public_key()
        .map(|pk| pk.as_hex())
        .map_err(|err| AppAuthHandlerError::from_source(Box::new(err)))?;

    let purchase_order_contract_registry_txn = CreateContractRegistryActionBuilder::new()
        .with_name(String::from(&purchase_order_contract.metadata.name))
        .with_owners(vec![public_key.clone()])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;

    let purchase_order_contract_txn = make_upload_contract_txn(
        signer,
        &purchase_order_contract,
        GRID_PURCHASE_ORDER_NAMESPACE,
    )?;

    let purchase_order_namespace_registry_txn = CreateNamespaceRegistryActionBuilder::new()
        .with_namespace(GRID_PURCHASE_ORDER_NAMESPACE.into())
        .with_owners(vec![public_key.clone()])
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?;
    let purchase_order_namespace_permissions_txn =
        CreateNamespaceRegistryPermissionActionBuilder::new()
            .with_namespace(GRID_PURCHASE_ORDER_NAMESPACE.into())
            .with_contract_name(purchase_order_contract.metadata.name.clone())
            .with_read(true)
            .with_write(true)
            .into_payload_builder()?
            .into_transaction_builder(signer)?
            .build(signer)?;
    let purchase_order_pike_namespace_permissions_txn =
        CreateNamespaceRegistryPermissionActionBuilder::new()
            .with_namespace(PIKE_NAMESPACE.into())
            .with_contract_name(purchase_order_contract.metadata.name)
            .with_read(true)
            .with_write(true)
            .into_payload_builder()?
            .into_transaction_builder(signer)?
            .build(signer)?;

    txns.append(&mut vec![
        purchase_order_contract_registry_txn,
        purchase_order_contract_txn,
        purchase_order_namespace_registry_txn,
        purchase_order_namespace_permissions_txn,
        purchase_order_pike_namespace_permissions_txn,
    ]);

    Ok(())
}

#[cfg(feature = "pike")]
fn make_upload_contract_txn(
    signer: &dyn Signer,
    contract: &SmartContractArchive,
    contract_prefix: &str,
) -> Result<Transaction, AppAuthHandlerError> {
    let action_addresses = vec![PIKE_NAMESPACE.into(), contract_prefix.into()];
    Ok(CreateContractActionBuilder::new()
        .with_name(contract.metadata.name.clone())
        .with_version(contract.metadata.version.clone())
        .with_inputs(action_addresses.clone())
        .with_outputs(action_addresses)
        .with_contract(contract.contract.clone())
        .into_payload_builder()?
        .into_transaction_builder(signer)?
        .build(signer)?)
}
