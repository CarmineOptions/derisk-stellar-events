use std::fmt;

use crate::schema::events;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use stellar_xdr::next::{ContractEvent, ContractEventBody};

pub enum StellarNetwork {
    Futurenet,
    Testnet,
    Mainnet,
}

impl fmt::Display for StellarNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StellarNetwork::Futurenet => write!(f, "stellar-futurenet"),
            StellarNetwork::Testnet => write!(f, "stellar-testnet"),
            StellarNetwork::Mainnet => write!(f, "stellar-mainnet"),
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, PartialEq, Selectable)]
#[diesel(table_name = events)]
pub struct DeriskEvent {
    pub id: String,
    pub contract_id: String,
    pub index: i32,
    pub sequence: i64,
    pub topics: serde_json::Value,
    pub data: serde_json::Value,
}

fn generate_id(sequence: i64, index: usize) -> String {
    format!("{}_{}", sequence, index)
}

pub fn construct_derisk_event(
    contract_event: &ContractEvent,
    sequence: u32,
    index: usize,
) -> Option<DeriskEvent> {
    let contract_id = match &contract_event.contract_id {
        Some(id) => format!("{}", &id),
        // no events without contract_id
        None => return None,
    };
    let (topics, data) = match &contract_event.body {
        ContractEventBody::V0(body) => (
            serde_json::from_str(&serde_json::to_string(&body.topics).unwrap()).unwrap(),
            serde_json::from_str(&serde_json::to_string(&body.data).unwrap()).unwrap(),
        ),
    };

    Some(DeriskEvent {
        id: generate_id(sequence as i64, index),
        contract_id,
        index: index as i32,
        sequence: sequence as i64,
        topics,
        data,
    })
}

// StellarX pools
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StellarXPool {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "_links")]
    pub links: Links,
    #[serde(rename = "fee_bp")]
    pub fee_bp: i64,
    pub reserves: Vec<Resef>,
    #[serde(rename = "paging_token")]
    pub paging_token: String,
    #[serde(rename = "total_shares")]
    pub total_shares: String,
    #[serde(rename = "total_trustlines")]
    pub total_trustlines: String,
    #[serde(rename = "last_modified_time")]
    pub last_modified_time: String,
    #[serde(rename = "last_modified_ledger")]
    pub last_modified_ledger: i64,
    pub liquidity: f64,
    #[serde(rename = "volume_24h")]
    pub volume_24h: f64,
    #[serde(rename = "volume_7d")]
    pub volume_7d: f64,
    #[serde(rename = "fee_24h")]
    pub fee_24h: f64,
    #[serde(rename = "fee_1y")]
    pub fee_1y: f64,
    #[serde(rename = "fee_aqua")]
    pub fee_aqua: f64,
    #[serde(rename = "assets_order")]
    pub assets_order: String,
    #[serde(rename = "asset_1_domain")]
    pub asset_1_domain: String,
    #[serde(rename = "asset_2_domain")]
    pub asset_2_domain: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: SelfField,
    pub operations: Operations,
    pub transactions: Transactions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfField {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operations {
    pub href: String,
    pub templated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub href: String,
    pub templated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resef {
    pub asset: String,
    pub amount: f64,
    pub asset_type: String,
    pub asset_code: String,
    pub asset_issuer: String,
    pub image: String,
    pub rate: f64,
    pub info: Info,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub org_url: String,
    pub org_url_domain: String,
}
