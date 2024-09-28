use crate::domain::common::latlng::LatLng;

pub enum Token {
    Prefecture(Prefecture),
    City(City),
    Town(Town),
    Rest(Rest),
}

struct Prefecture {
    prefecture_name: String,
    representative_point: Option<LatLng>,
}

struct City {
    city_name: String,
    representative_point: Option<LatLng>,
}

struct Town {
    town_name: String,
    representative_point: Option<LatLng>,
}

struct Rest {
    rest: String,
    representative_point: Option<LatLng>,
}
