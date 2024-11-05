use crate::domain::common::latlng::LatLng;
use crate::domain::common::token::Token;
use crate::experimental::parser::Parser;
use crate::interactor::chimei_ruiju::{ChimeiRuijuInteractor, ChimeiRuijuInteractorImpl};
use crate::tokenizer::Tokenizer;
use std::option::Option;

impl Parser {
    pub(crate) async fn parse_with_chimeiruiju(
        &self,
        address: &str,
    ) -> (Vec<Token>, Option<LatLng>) {
        let interactor = ChimeiRuijuInteractorImpl::default();
        let tokenizer = Tokenizer::new(address);
        let mut lat_lng: Option<LatLng> = None;

        // 都道府県名の検出
        let (prefecture, tokenizer) = match tokenizer.read_prefecture() {
            Ok(found) => found,
            Err(not_found) => {
                if self.options.verbose {
                    log::error!("都道府県名の検出に失敗しました")
                }
                return (not_found.tokens, lat_lng);
            }
        };

        // 都道府県マスタの取得
        let prefecture_master = match interactor.get_prefecture_master(&prefecture).await {
            Ok(result) => {
                lat_lng.replace(result.coordinate.to_lat_lng());
                result
            }
            Err(error) => {
                if self.options.verbose {
                    log::error!("{}", error)
                }
                return (tokenizer.finish().tokens, lat_lng);
            }
        };
        // 市区町村名の検出
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
                            return (not_found.tokens, lat_lng);
                        }
                    }
                } else {
                    if self.options.verbose {
                        log::error!("市区町村名の検出に失敗しました")
                    }
                    return (not_found.finish().tokens, lat_lng);
                }
            }
        };

        // 市区町村マスタの取得
        let city_master = match interactor.get_city_master(&prefecture, &city_name).await {
            Ok(result) => {
                lat_lng.replace(result.coordinate.to_lat_lng());
                result
            }
            Err(error) => {
                if self.options.verbose {
                    log::error!("{}", error)
                }
                return (tokenizer.finish().tokens, lat_lng);
            }
        };
        // 町名の検出
        let (town_name, tokenizer) = match tokenizer.read_town(city_master.towns) {
            Ok(found) => found,
            Err(not_found) => {
                if self.options.verbose {
                    log::error!("町名の検出に失敗しました")
                }
                return (not_found.tokens, lat_lng);
            }
        };

        // 町村マスタの取得
        if let Ok(town_master) = interactor
            .get_town_master(&prefecture, &city_name, &town_name)
            .await
        {
            lat_lng.replace(town_master.coordinate.to_lat_lng());
        };

        (tokenizer.finish().tokens, lat_lng)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::common::token::Token;
    use crate::experimental::parser::{DataSource, Parser, ParserOptions};

    #[tokio::test]
    async fn 都道府県名が誤っている場合() {
        let parser = Parser {
            options: ParserOptions {
                data_source: DataSource::ChimeiRuiju,
                correct_incomplete_city_names: false,
                verbose: false,
            },
        };
        let (tokens, _) = parser
            .parse_with_chimeiruiju("奈川県横浜市磯子区洋光台3-10-3")
            .await;
        assert_eq!(
            tokens,
            vec![Token::Rest("奈川県横浜市磯子区洋光台3-10-3".to_string())]
        )
    }

    #[tokio::test]
    async fn 市区町村名が誤っている場合() {
        let parser = Parser {
            options: ParserOptions {
                data_source: DataSource::ChimeiRuiju,
                correct_incomplete_city_names: false,
                verbose: false,
            },
        };
        let (tokens, _) = parser
            .parse_with_chimeiruiju("神奈川県横浜県磯子市洋光台3-10-3")
            .await;
        assert_eq!(
            tokens,
            vec![
                Token::Prefecture("神奈川県".to_string()),
                Token::Rest("横浜県磯子市洋光台3-10-3".to_string())
            ]
        )
    }

    #[tokio::test]
    async fn 町名が誤っている場合() {
        let parser = Parser {
            options: ParserOptions {
                data_source: DataSource::ChimeiRuiju,
                correct_incomplete_city_names: false,
                verbose: false,
            },
        };
        let (tokens, _) = parser
            .parse_with_chimeiruiju("神奈川県横浜市磯子区陽光台3-10-3")
            .await;
        assert_eq!(
            tokens,
            vec![
                Token::Prefecture("神奈川県".to_string()),
                Token::City("横浜市磯子区".to_string()),
                Token::Rest("陽光台3-10-3".to_string())
            ]
        )
    }

    #[tokio::test]
    async fn パースに成功した場合() {
        let parser = Parser {
            options: ParserOptions {
                data_source: DataSource::ChimeiRuiju,
                correct_incomplete_city_names: false,
                verbose: false,
            },
        };
        let (tokens, _) = parser
            .parse_with_chimeiruiju("神奈川県横浜市磯子区洋光台3-10-3")
            .await;
        assert_eq!(
            tokens,
            vec![
                Token::Prefecture("神奈川県".to_string()),
                Token::City("横浜市磯子区".to_string()),
                Token::Town("洋光台三丁目".to_string()),
                Token::Rest("10-3".to_string())
            ]
        )
    }
}
