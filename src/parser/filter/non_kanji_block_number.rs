use crate::parser::filter::Filter;
use crate::util::converter::JapaneseNumber;

pub struct NonKanjiBlockNumberFilter {}

impl Filter for NonKanjiBlockNumberFilter {
    fn apply(self, input: String) -> String {
        let expression = regex::Regex::new(r"\D+(?<block_number>\d+)丁目").unwrap();
        match expression.captures(&input) {
            Some(captures) => {
                let capture_block_number = &captures.name("block_number").unwrap().as_str();
                let block_number = capture_block_number.parse::<i32>().unwrap();
                input.replacen(
                    capture_block_number,
                    block_number.to_japanese_form().unwrap().as_str(),
                    1,
                )
            }
            None => input,
        }
    }
}
