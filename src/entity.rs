use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Prefecture {
    pub name: String,
    pub cities: Vec<String>,
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
    pub lat: f32,
    pub lng: f32,
}

#[derive(Serialize)]
pub struct ParsedAddress {
    pub prefecture: String,
    pub city: String,
    pub town: String,
    pub rest: String,
}

impl ParsedAddress {
    pub fn new(prefecture_name: &str, city_name: &str, town_name: &str, rest_name: &str) -> Self {
        ParsedAddress {
            prefecture: prefecture_name.to_string(),
            city: city_name.to_string(),
            town: town_name.to_string(),
            rest: rest_name.to_string(),
        }
    }
}
