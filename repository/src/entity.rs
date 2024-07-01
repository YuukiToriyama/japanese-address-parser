use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PrefectureMaster {
    /// 都道府県名
    pub name: String,
    /// 市区町村名リスト
    pub cities: Vec<String>,
    /// 代表点の緯度経度
    pub coordinate: Coordinate,
}

#[derive(Deserialize, Debug)]
pub struct CityMaster {
    /// 市区町村名
    pub name: String,
    /// 町名リスト
    pub towns: Vec<String>,
    /// 緯度経度
    pub coordinate: Coordinate,
}

#[derive(Deserialize, Debug)]
pub struct Coordinate {
    /// 緯度
    pub latitude: f64,
    /// 経度
    pub longitude: f64,
}
