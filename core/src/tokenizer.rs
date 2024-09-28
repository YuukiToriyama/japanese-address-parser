pub(crate) mod read_city;
pub(crate) mod read_city_with_county_name_completion;
pub(crate) mod read_prefecture;
pub(crate) mod read_town;

use crate::domain::common::token::Token;
use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct Init;
#[derive(Debug)]
pub(crate) struct PrefectureNameFound;
#[derive(Debug)]
pub(crate) struct CityNameFound;
#[derive(Debug)]
pub(crate) struct CityNameNotFound;
#[derive(Debug)]
pub(crate) struct TownNameFound;
#[derive(Debug)]
pub(crate) struct End;

#[derive(Debug)]
pub struct Tokenizer<State> {
    pub(crate) tokens: Vec<Token>,
    pub(crate) rest: String,
    _state: PhantomData<State>,
}

impl<T> Tokenizer<T> {
    fn get_prefecture_name(&self) -> Option<&str> {
        for token in &self.tokens {
            if let Token::Prefecture(prefecture) = token {
                return Some(&prefecture.prefecture_name);
            };
        }
        None
    }
}
