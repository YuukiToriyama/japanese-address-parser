use crate::domain::common::latlng::LatLng;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PrefectureMaster {
    /// 都道府県名
    pub(crate) name: String,
    /// 市区町村名リスト
    pub(crate) cities: Vec<String>,
    /// 代表点の緯度経度
    pub(crate) coordinate: Coordinate,
}

#[derive(Deserialize, Debug)]
pub struct CityMaster {
    /// 市区町村名
    pub(crate) name: String,
    /// 町名リスト
    pub(crate) towns: Vec<String>,
    /// 緯度経度
    pub(crate) coordinate: Coordinate,
}

#[derive(Deserialize, Debug)]
pub struct TownMaster {
    /// 町名
    pub(crate) name: String,
    /// 街区リスト
    blocks: Vec<Block>,
    /// 緯度経度
    pub(crate) coordinate: Coordinate,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Block {
    /// 小字・通称名
    koaza: String,
    /// 街区符号・地番
    block_number: String,
    /// 住居表示の有無
    residential_address_indication: bool,
    /// 緯度経度
    coordinate: Coordinate,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Coordinate {
    /// 緯度
    pub(crate) latitude: f64,
    /// 経度
    pub(crate) longitude: f64,
}

impl Coordinate {
    pub(crate) fn to_lat_lng(&self) -> LatLng {
        LatLng {
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}
