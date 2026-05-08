use crate::domain::common::token::Token;
use crate::domain::geolonia::entity::Address;
use crate::domain::geolonia::error::Error;
use crate::tokenizer::{End, Tokenizer};
use serde::Serialize;

/// Represents the result of an address parsing.
#[derive(Serialize, PartialEq, Debug)]
pub struct ParseResult {
    pub address: Address,
    pub error: Option<Error>,
}

impl From<Tokenizer<End>> for Address {
    fn from(value: Tokenizer<End>) -> Self {
        let mut address = Address::new("", "", "", "");
        for token in value.tokens {
            match token {
                Token::Prefecture(prefecture_name) => address.prefecture = prefecture_name,
                Token::City(city_name) => address.city = city_name,
                Token::Town(town_name) => address.town = town_name,
                Token::Rest(rest) => address.rest = rest,
            }
        }
        address
    }
}
