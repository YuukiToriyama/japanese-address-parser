use crate::domain::common::latlng::LatLng;

#[derive(Debug)]
pub enum Token {
    Prefecture(Prefecture),
    City(City),
    Town(Town),
    Rest(Rest),
}

#[derive(Debug)]
pub(crate) struct Prefecture {
    prefecture_name: String,
    representative_point: Option<LatLng>,
}

#[derive(Debug)]
pub(crate) struct City {
    city_name: String,
    representative_point: Option<LatLng>,
}

#[derive(Debug)]
pub(crate) struct Town {
    town_name: String,
    representative_point: Option<LatLng>,
}

#[derive(Debug)]
pub(crate) struct Rest {
    rest: String,
    representative_point: Option<LatLng>,
}
