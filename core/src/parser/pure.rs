use crate::domain::geolonia::entity::Address;
use crate::domain::geolonia::error::{Error, ParseErrorKind};
use crate::parser::ParseResult;
use crate::tokenizer::{CityNameFound, Init, PrefectureNameFound, Tokenizer};

type PrefectureName = String;
type CityName = String;

pub(crate) enum State {
    Init(Tokenizer<Init>),
    WaitPrefectureMasterData(Tokenizer<PrefectureNameFound>, PrefectureName),
    WaitCityMasterData(Tokenizer<CityNameFound>),
    Temporary,
}

pub(crate) enum PureParserAction {
    RequestCityNameList(PrefectureName),
    RequestTownNameList(PrefectureName, CityName),
    Done(ParseResult),
}

pub(crate) struct PureParser {
    state: State,
    input: Option<Vec<String>>,
}

impl PureParser {
    pub fn new(address: &str) -> Self {
        Self {
            state: State::Init(Tokenizer::new(address)),
            input: None,
        }
    }

    pub fn provide_input(&mut self, data: Vec<String>) {
        self.input = Some(data);
    }

    pub fn advance(&mut self) -> PureParserAction {
        let current_state = std::mem::replace(&mut self.state, State::Temporary);
        let input = self.input.take();

        match current_state {
            State::Init(tokenizer) => match tokenizer.read_prefecture() {
                Ok((pref, next_tokenizer)) => {
                    let pref_name = pref.name_ja().to_string();
                    self.state = State::WaitPrefectureMasterData(next_tokenizer, pref_name.clone());
                    PureParserAction::RequestCityNameList(pref_name)
                }
                Err(tokenizer) => PureParserAction::Done(ParseResult {
                    address: Address::from(tokenizer),
                    error: Some(Error::new_parse_error(ParseErrorKind::Prefecture)),
                }),
            },

            State::WaitPrefectureMasterData(tokenizer, pref_name) => {
                let city_names = input.expect("city name list is required");
                match tokenizer.read_city(&city_names) {
                    Ok((city_name, next_tokenizer)) => {
                        self.state = State::WaitCityMasterData(next_tokenizer);
                        PureParserAction::RequestTownNameList(pref_name.clone(), city_name)
                    }
                    Err(tokenizer) => {
                        match tokenizer.read_city_with_county_name_completion(&city_names) {
                            Ok((city_name, next_tokenizer))
                                if cfg!(feature = "city-name-correction") =>
                            {
                                self.state = State::WaitCityMasterData(next_tokenizer);
                                PureParserAction::RequestTownNameList(pref_name.clone(), city_name)
                            }
                            _ => PureParserAction::Done(ParseResult {
                                address: Address::from(tokenizer.finish()),
                                error: Some(Error::new_parse_error(ParseErrorKind::City)),
                            }),
                        }
                    }
                }
            }

            State::WaitCityMasterData(tokenizer) => {
                let town_names = input.expect("town name list is required");
                match tokenizer.read_town(town_names) {
                    Ok((_, next_tokenizer)) => PureParserAction::Done(ParseResult {
                        address: Address::from(next_tokenizer.finish()),
                        error: None,
                    }),
                    Err(tokenizer) => PureParserAction::Done(ParseResult {
                        address: Address::from(tokenizer),
                        error: Some(Error::new_parse_error(ParseErrorKind::Town)),
                    }),
                }
            }

            State::Temporary => unreachable!(),
        }
    }

    /// IOエラーなど、途中で解析を中断してエラーを返す場合に使用する
    pub fn abort(self, error: Error) -> ParseResult {
        let current_state = self.state;
        let tokenizer = match current_state {
            State::Init(t) => t.finish(),
            State::WaitPrefectureMasterData(t, _) => t.finish(),
            State::WaitCityMasterData(t) => t.finish(),
            State::Temporary => unreachable!(),
        };

        ParseResult {
            address: Address::from(tokenizer),
            error: Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::geolonia::error::{ApiErrorKind, Error};
    use crate::parser::pure::{PureParser, PureParserAction, State};

    #[test]
    fn new() {
        let pure_parser = PureParser::new("東京都杉並区阿佐谷南1丁目15番1号");
        assert!(matches!(pure_parser.state, State::Init(_)));
    }

    #[test]
    fn provide_input() {
        let mut pure_parser = PureParser::new("東京都杉並区阿佐谷南1丁目15番1号");
        assert!(pure_parser.input.is_none());

        pure_parser.provide_input(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert!(pure_parser.input.is_some());
        assert_eq!(
            pure_parser.input.unwrap(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn advance() {
        let mut pure_parser = PureParser::new("東京都杉並区阿佐谷南1丁目15番1号");
        assert!(matches!(pure_parser.state, State::Init(_)));

        let action = pure_parser.advance();
        assert!(matches!(action, PureParserAction::RequestCityNameList(_)));
        assert!(matches!(
            pure_parser.state,
            State::WaitPrefectureMasterData(_, _)
        ));

        pure_parser.provide_input(vec![
            "杉並区".to_string(),
            "板橋区".to_string(),
            "中野区".to_string(),
        ]);
        let action = pure_parser.advance();
        assert!(matches!(
            action,
            PureParserAction::RequestTownNameList(_, _)
        ));
        assert!(matches!(pure_parser.state, State::WaitCityMasterData(_)));

        pure_parser.provide_input(vec![
            "阿佐谷南一丁目".to_string(),
            "阿佐谷南二丁目".to_string(),
            "阿佐谷南三丁目".to_string(),
        ]);
        let action = pure_parser.advance();
        assert!(matches!(action, PureParserAction::Done(_)));
    }

    #[test]
    fn abort() {
        let pure_parser = PureParser::new("東京都杉並区阿佐谷南1丁目15番1号");
        let result = pure_parser.abort(Error::new_api_error(ApiErrorKind::Fetch(
            "hoge".to_string(),
        )));

        assert_eq!(result.address.rest, "東京都杉並区阿佐谷南1丁目15番1号");
        assert!(result.error.is_some());
    }
}
