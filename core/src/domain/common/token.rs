use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Prefecture(String),
    City(String),
    Town(String),
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

pub(crate) fn append_token(tokens: &[Token], token: Token) -> Vec<Token> {
    [tokens.to_owned(), vec![token]].concat()
}

#[cfg(test)]
mod tests {
    use crate::domain::common::token::Token;

    #[test]
    fn sort_token_vector() {
        let mut tokens = vec![
            Token::Rest("2-1".to_string()),
            Token::City("小金井市".to_string()),
            Token::Prefecture("東京都".to_string()),
            Token::Town("貫井北町四丁目".to_string()),
        ];
        tokens.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            tokens,
            vec![
                Token::Prefecture("東京都".to_string()),
                Token::City("小金井市".to_string()),
                Token::Town("貫井北町四丁目".to_string()),
                Token::Rest("2-1".to_string()),
            ]
        );
    }
}
