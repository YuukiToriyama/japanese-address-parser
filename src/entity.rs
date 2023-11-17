pub struct Prefecture {
    pub name: String,
    pub cities: Vec<String>,
}

pub struct City {
    pub name: String,
    pub towns: Vec<Town>,
}

pub struct Town {
    pub name: String,
    pub koaza: String,
    pub lat: f32,
    pub lng: f32,
}
