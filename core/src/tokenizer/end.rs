use crate::domain::common::token::{append_token, Token};
use crate::tokenizer::{End, Tokenizer, TownNameFound};
use std::marker::PhantomData;

impl Tokenizer<TownNameFound> {
    pub(crate) fn finish(&self) -> Tokenizer<End> {
        Tokenizer {
            tokens: append_token(&self.tokens, Token::Rest(self.rest.clone())),
            rest: "".to_string(),
            _state: PhantomData::<End>,
        }
    }
}
