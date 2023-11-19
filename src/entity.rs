use serde::Deserialize;

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

pub struct ParsedAddress {
    pub prefecture: String,
    pub city: String,
    pub town: String,
    pub rest: String,
}
