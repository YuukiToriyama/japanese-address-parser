use crate::domain::common::latlng::LatLng;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Prefecture(Prefecture),
    City(City),
    Town(Town),
    Rest(Rest),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Prefecture {
    pub(crate) prefecture_name: String,
    pub(crate) representative_point: Option<LatLng>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct City {
    city_name: String,
    representative_point: Option<LatLng>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Town {
    town_name: String,
    representative_point: Option<LatLng>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Rest {
    rest: String,
    representative_point: Option<LatLng>,
}
