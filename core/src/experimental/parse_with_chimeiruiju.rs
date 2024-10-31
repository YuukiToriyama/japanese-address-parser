use crate::domain::common::token::Token;
use crate::experimental::parser::Parser;
use crate::interactor::{ChimeiRuijuInteractor, ChimeiRuijuInteractorImpl};
use crate::tokenizer::Tokenizer;

impl Parser {
    pub(crate) async fn parse_with_chimeiruiju(&self, address: &str) -> Vec<Token> {
        let interactor = ChimeiRuijuInteractorImpl::default();
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
        let prefecture_master = match interactor.get_prefecture_master(&prefecture).await {
            Ok(result) => result,
            Err(error) => {
                if self.options.verbose {
                    log::error!("{}", error)
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
        let city_master = match interactor.get_city_master(&prefecture, &city_name).await {
            Ok(result) => result,
            Err(error) => {
                if self.options.verbose {
                    log::error!("{}", error)
                }
                return tokenizer.finish().tokens;
            }
        };
        let (_, tokenizer) = match tokenizer.read_town(city_master.towns) {
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
