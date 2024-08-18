pub(crate) mod read_city;
pub(crate) mod read_city_with_county_name_completion;
pub(crate) mod read_prefecture;
pub(crate) mod read_town;

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
    input: String,
    pub(crate) prefecture_name: Option<String>,
    pub(crate) city_name: Option<String>,
    pub(crate) town_name: Option<String>,
    pub(crate) rest: String,
    _state: PhantomData<State>,
}
