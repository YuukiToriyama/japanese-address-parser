use crate::util::sequence_matcher::SequenceMatcher;
use nom::bytes::complete::{is_a, is_not};
use nom::combinator::rest;
use nom::error::Error;
use nom::sequence::tuple;

pub struct VagueExpressionAdapter;

impl VagueExpressionAdapter {
    pub fn apply(self, input: &str, region_name_list: &Vec<String>) -> Option<(String, String)> {
        match SequenceMatcher::get_most_similar_match(input, region_name_list, None) {
            Ok(highest_match) => {
                match tuple((
                    is_not::<&str, &str, Error<&str>>("町村"),
                    is_a("町村"),
                    rest,
                ))(input)
                {
                    Ok((_, separated)) => {
                        let (_, _, rest) = separated;
                        Some((rest.to_string(), highest_match))
                    }
                    Err(..) => None,
                }
            }
            Err(..) => None,
        }
    }
}
