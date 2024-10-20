use crate::api::AsyncApi;
use crate::domain::common::token::Token;
use crate::domain::geolonia::error::Error;
use crate::experimental::parser::{DataSource, Parser};
use crate::tokenizer::{
    CityNameFound, CityNameNotFound, End, PrefectureNameFound, Tokenizer, TownNameFound,
};
use jisx0401::Prefecture;

impl Parser {
    #[inline]
    pub(crate) async fn parse_with_geolonia(&self, address: &str) -> Vec<Token> {
        let geolonia_api = AsyncApi::default();
        let tokenizer = Tokenizer::new(address);

        // 都道府県名の検出
        let (prefecture, tokenizer) = match tokenizer.read_prefecture() {
            Ok(found) => found,
            Err(not_found) => {
                if self.options.verbose {
                    log::error!("都道府県名の検出に失敗しました")
                }
                return not_found.tokens;
            }
        };

        // 市区町村名の検出
        let prefecture_master = match geolonia_api
            .get_prefecture_master(prefecture.name_ja())
            .await
        {
            Ok(result) => result,
            Err(error) => {
                if self.options.verbose {
                    log::error!("{}", error.error_message)
                }
                return tokenizer.finish().tokens;
            }
        };
        let (city_name, tokenizer) = match tokenizer.read_city(&prefecture_master.cities) {
            Ok(found) => found,
            Err(not_found) => {
                if self.options.correct_incomplete_city_names {
                    match not_found.read_city_with_county_name_completion(&prefecture_master.cities)
                    {
                        Ok(result) => result,
                        Err(not_found) => {
                            if self.options.verbose {
                                log::error!("市区町村名の検出に失敗しました")
                            }
                            return not_found.tokens;
                        }
                    }
                } else {
                    if self.options.verbose {
                        log::error!("市区町村名の検出に失敗しました")
                    }
                    return not_found.finish().tokens;
                }
            }
        };

        // 町名の検出
        let city_master = match geolonia_api
            .get_city_master(prefecture.name_ja(), &city_name)
            .await
        {
            Ok(result) => result,
            Err(error) => {
                if self.options.verbose {
                    log::error!("{}", error.error_message)
                }
                return tokenizer.finish().tokens;
            }
        };
        let (_, tokenizer) =
            match tokenizer.read_town(city_master.towns.iter().map(|x| x.name.clone()).collect()) {
                Ok(found) => found,
                Err(not_found) => {
                    if self.options.verbose {
                        log::error!("町名の検出に失敗しました")
                    }
                    return not_found.tokens;
                }
            };

        tokenizer.finish().tokens
    }
}
