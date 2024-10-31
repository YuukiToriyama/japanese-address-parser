use crate::domain::common::latlng::LatLng;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Prefecture(Prefecture),
    City(City),
    Town(Town),
    Rest(String),
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Token::Prefecture(_) => match other {
                Token::Prefecture(_) => Some(Equal),
                Token::City(_) => Some(Less),
                Token::Town(_) => Some(Less),
                Token::Rest(_) => Some(Less),
            },
            Token::City(_) => match other {
                Token::Prefecture(_) => Some(Greater),
                Token::City(_) => Some(Equal),
                Token::Town(_) => Some(Less),
                Token::Rest(_) => Some(Less),
            },
            Token::Town(_) => match other {
                Token::Prefecture(_) => Some(Greater),
                Token::City(_) => Some(Greater),
                Token::Town(_) => Some(Equal),
                Token::Rest(_) => Some(Less),
            },
            Token::Rest(_) => match other {
                Token::Prefecture(_) => Some(Greater),
                Token::City(_) => Some(Greater),
                Token::Town(_) => Some(Greater),
                Token::Rest(_) => Some(Equal),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Prefecture {
    pub(crate) prefecture_name: String,
    pub(crate) representative_point: Option<LatLng>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct City {
    pub(crate) city_name: String,
    pub(crate) representative_point: Option<LatLng>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Town {
    pub(crate) town_name: String,
    pub(crate) representative_point: Option<LatLng>,
}

pub(crate) fn append_token(tokens: &[Token], token: Token) -> Vec<Token> {
    [tokens.to_owned(), vec![token]].concat()
}

#[cfg(test)]
mod tests {
    use crate::domain::common::token::{City, Prefecture, Token, Town};

    #[test]
    fn sort_token_vector() {
        let mut tokens = vec![
            Token::Rest("2-1".to_string()),
            Token::City(City {
                city_name: "小金井市".to_string(),
                representative_point: None,
            }),
            Token::Prefecture(Prefecture {
                prefecture_name: "東京都".to_string(),
                representative_point: None,
            }),
            Token::Town(Town {
                town_name: "貫井北町四丁目".to_string(),
                representative_point: None,
            }),
        ];
        tokens.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            tokens,
            vec![
                Token::Prefecture(Prefecture {
                    prefecture_name: "東京都".to_string(),
                    representative_point: None,
                }),
                Token::City(City {
                    city_name: "小金井市".to_string(),
                    representative_point: None,
                }),
                Token::Town(Town {
                    town_name: "貫井北町四丁目".to_string(),
                    representative_point: None,
                }),
                Token::Rest("2-1".to_string()),
            ]
        );
    }
}
