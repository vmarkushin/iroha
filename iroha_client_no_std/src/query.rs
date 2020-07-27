//! This module contains query related Iroha functionality.

use crate::crypto::Signature;
use crate::{account, domain};
use alloc::{string::String, vec::Vec};
use parity_scale_codec::{Decode, Encode};
use alloc::{
    string::String,
    vec::Vec,
};

/// I/O ready structure to send queries.
#[derive(Debug, Encode, Decode, Clone)]
pub struct QueryRequest {
    /// Timestamp of the query creation.
    pub timestamp: String,
    /// Query definition.
    pub query: IrohaQuery,
}

/// I/O ready structure to send queries.
#[derive(Debug, Encode, Decode)]
pub struct SignedQueryRequest {
    /// Timestamp of the query creation.
    pub timestamp: String,
    /// Signature of the client who sends this query.
    pub signature: Signature,
    /// Query definition.
    pub query: IrohaQuery,
}

/// Enumeration of all legal Iroha Queries.
#[derive(Clone, Debug, Encode, Decode)]
pub enum IrohaQuery {
    /// Query all Assets related to the Account.
    GetAccountAssets(asset::query::GetAccountAssets),
    /// Query all Assets with defined Definition related to the Account.
    GetAccountAssetsWithDefinition(asset::query::GetAccountAssetsWithDefinition),
    /// Query Account information.
    GetAccount(account::query::GetAccount),
    /// Query Domain information.
    GetDomain(domain::query::GetDomain),
}

/// Result of queries execution.
#[derive(Debug, Encode, Decode)]
pub enum QueryResult {
    /// Query all Assets related to the Account result.
    GetAccountAssets(asset::query::GetAccountAssetsResult),
    /// Query all Assets with defined Definition related to the Account.
    GetAccountAssetsWithDefinition(asset::query::GetAccountAssetsWithDefinitionResult),
    /// Query Account information.
    GetAccount(account::query::GetAccountResult),
    /// Query Domain information.
    GetDomain(domain::query::GetDomainResult),
}
