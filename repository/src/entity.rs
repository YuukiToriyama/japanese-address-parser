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
pub struct TownMaster {
    /// 町名
    pub name: String,
    /// 街区リスト
    pub blocks: Vec<Block>,
    /// 緯度経度
    pub coordinate: Coordinate,
}

#[derive(Deserialize, Debug)]
pub struct Block {
    /// 小字・通称名
    pub koaza: String,
    /// 街区符号・地番
    pub block_number: String,
    /// 住居表示の有無
    pub residential_address_indication: bool,
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
