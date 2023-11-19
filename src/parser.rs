use crate::api::Api;
use crate::entity::ParsedAddress;
use crate::parser::read_city::read_city;
use crate::parser::read_prefecture::read_prefecture;
use crate::parser::read_town::read_town;

mod read_city;
mod read_prefecture;
mod read_town;

pub async fn parse<T: Api>(api: T, input: &str) -> ParsedAddress {
    // 都道府県を特定
    let (rest, prefecture_name) = read_prefecture(input).unwrap();
    // その都道府県の市町村名リストを取得
    let prefecture = api.get_prefecture_master(prefecture_name).await.unwrap();
    // 市町村名を特定
    let (rest, city_name) = read_city(rest, prefecture).unwrap();
    // その市町村の町名リストを取得
    let city = api.get_city_master(prefecture_name, city_name).await.unwrap();
    // 町名を特定
    let (rest, town_name) = read_town(rest, city).unwrap();

    ParsedAddress {
        prefecture: prefecture_name.to_string(),
        city: city_name.to_string(),
        town: town_name.to_string(),
        rest: rest.to_string(),
    }
}