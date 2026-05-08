use crate::domain::geolonia::entity::Address;
use crate::domain::geolonia::error::Error;
use serde::Serialize;

/// Represents the result of an address parsing.
#[derive(Serialize, PartialEq, Debug)]
pub struct ParseResult {
    pub address: Address,
    pub error: Option<Error>,
}
