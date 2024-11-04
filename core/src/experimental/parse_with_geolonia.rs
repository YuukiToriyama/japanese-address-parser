use crate::domain::common::token::Token;
use crate::experimental::parser::Parser;
use crate::interactor::geolonia::{GeoloniaInteractor, GeoloniaInteractorImpl};
use crate::tokenizer::Tokenizer;

impl Parser {
    #[inline]
    pub(crate) async fn parse_with_geolonia(&self, address: &str) -> Vec<Token> {
        let interactor = GeoloniaInteractorImpl::default();
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
        let prefecture_master = match interactor.get_prefecture_master(prefecture.name_ja()).await {
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
        let city_master = match interactor
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

#[cfg(test)]
mod tests {
    use crate::domain::common::token::Token;
    use crate::experimental::parser::{DataSource, Parser, ParserOptions};

    #[tokio::test]
    async fn 都道府県名が誤っている場合() {
        let parser = Parser {
            options: ParserOptions {
                data_source: DataSource::Geolonia,
                correct_incomplete_city_names: false,
                verbose: false,
            },
        };
        let result = parser
            .parse_with_geolonia("奈川県横浜市磯子区洋光台3-10-3")
            .await;
        assert_eq!(
            result,
            vec![Token::Rest("奈川県横浜市磯子区洋光台3-10-3".to_string())]
        )
    }

    #[tokio::test]
    async fn 市区町村名が誤っている場合() {
        let parser = Parser {
            options: ParserOptions {
                data_source: DataSource::Geolonia,
                correct_incomplete_city_names: false,
                verbose: false,
            },
        };
        let result = parser
            .parse_with_geolonia("神奈川県横浜県磯子市洋光台3-10-3")
            .await;
        assert_eq!(
            result,
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
                data_source: DataSource::Geolonia,
                correct_incomplete_city_names: false,
                verbose: false,
            },
        };
        let result = parser
            .parse_with_geolonia("神奈川県横浜市磯子区陽光台3-10-3")
            .await;
        assert_eq!(
            result,
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
                data_source: DataSource::Geolonia,
                correct_incomplete_city_names: false,
                verbose: false,
            },
        };
        let result = parser
            .parse_with_geolonia("神奈川県横浜市磯子区洋光台3-10-3")
            .await;
        assert_eq!(
            result,
            vec![
                Token::Prefecture("神奈川県".to_string()),
                Token::City("横浜市磯子区".to_string()),
                Token::Town("洋光台三丁目".to_string()),
                Token::Rest("10-3".to_string())
            ]
        )
    }
}
