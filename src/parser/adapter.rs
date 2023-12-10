use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;
use std::ops::Not;

pub fn adapt_variety_of_spelling(
    input: &str,
    region_name: &str,
    variety_of_spelling: Vec<&str>,
) -> Option<(String, String)> {
    if variety_of_spelling
        .iter()
        .all(|s| region_name.contains(s).not())
    {
        return None;
    }
    for permutation in variety_of_spelling.iter().permutations(2) {
        if region_name.contains(permutation[0]) {
            let edited_region_name = region_name.replace(permutation[0], permutation[1]);
            if let Ok((rest, _)) =
                tag::<&str, &str, VerboseError<&str>>(&edited_region_name).parse(input)
            {
                return Some((rest.to_string(), region_name.to_string()));
            };
        }
    }
    None
}

#[cfg(test)]
mod adapter_tests {
    use crate::parser::adapter::adapt_variety_of_spelling;

    #[test]
    fn adapt_variety_of_spelling_異字体への対応_薮田() {
        let correct_town_name = "薮田南二丁目";
        let variety_of_spelling = vec!["薮", "藪", "籔"];
        assert_eq!(
            adapt_variety_of_spelling("藪田南二丁目", correct_town_name, variety_of_spelling.clone()).unwrap().1,
            correct_town_name
        );
        assert_eq!(
            adapt_variety_of_spelling("籔田南二丁目", correct_town_name, variety_of_spelling.clone()).unwrap().1,
            correct_town_name
        );
    }
}