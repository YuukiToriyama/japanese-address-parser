use crate::domain::common::latlng::LatLng;

pub enum Token {
    Prefecture(Prefecture),
    City(City),
    Town(Town),
    Rest(Rest),
}

pub(crate) struct Prefecture {
    prefecture_name: String,
    representative_point: Option<LatLng>,
}

pub(crate) struct City {
    city_name: String,
    representative_point: Option<LatLng>,
}

pub(crate) struct Town {
    town_name: String,
    representative_point: Option<LatLng>,
}

pub(crate) struct Rest {
    rest: String,
    representative_point: Option<LatLng>,
}
