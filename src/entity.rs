use crate::err::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Debug)]
pub struct Prefecture {
    pub name: String,
    pub cities: Vec<String>,
}

impl Prefecture {
    // 主にテストコードで使用する
    pub fn new(prefecture_name: &str, city_names: Vec<&str>) -> Self {
        Prefecture {
            name: prefecture_name.to_string(),
            cities: city_names.iter().map(|s| s.to_string()).collect(),
        }
    }
}

pub struct City {
    pub name: String,
    pub towns: Vec<Town>,
}

#[derive(PartialEq, Deserialize)]
pub struct Town {
    #[serde(alias = "town")]
    pub name: String,
    pub koaza: String,
    // TODO: https://github.com/geolonia/japanese-addresses/issues/148 が解消されたらOptionを外すことができる
    pub lat: Option<f32>,
    pub lng: Option<f32>,
}

#[derive(Serialize)]
pub struct Address {
    pub prefecture: String,
    pub city: String,
    pub town: String,
    pub rest: String,
}

impl Address {
    pub fn new(prefecture_name: &str, city_name: &str, town_name: &str, rest_name: &str) -> Self {
        Address {
            prefecture: prefecture_name.to_string(),
            city: city_name.to_string(),
            town: town_name.to_string(),
            rest: rest_name.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ParseResult {
    pub address: Address,
    pub error: Option<Error>,
}
