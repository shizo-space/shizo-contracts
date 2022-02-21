use std::fmt;

use near_sdk::serde::{Deserialize, Serialize};

/// Enum that represents the data type of the EventLog.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    NftSale(Vec<NftSaleLog>),
}

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

/// An event log to capture token sale
///
/// Arguments
/// * `old_owner_id`: "owner.near"
/// * `new_owner_id`: "receiver.near"
/// * `token_ids`: ["1", "12345abc"]
/// * `price`: [1, 2, 3]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftSaleLog {
    pub old_owner_id: String,
    pub new_owner_id: String,
    pub token_ids: Vec<String>,
    pub price: u128,
}
