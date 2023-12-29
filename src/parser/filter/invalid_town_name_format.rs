use crate::parser::filter::Filter;
use crate::util::converter::JapaneseNumber;
use regex::Regex;

pub struct InvalidTownNameFormatFilter {}

impl Filter for InvalidTownNameFormatFilter {
    fn apply(self, input: String) -> String {
        extract_town_name(&input).unwrap_or_else(|| input)
    }
}

fn extract_town_name(input: &str) -> Option<String> {
    let expression =
        Regex::new(r"^(?<town_name>\D+)(?<block_number>\d+)[-ー－]*(?<rest>.*)$").unwrap();
    let captures = expression.captures(input)?;
    let town_name = if let Some(matched) = captures.name("town_name") {
        matched.as_str()
    } else {
        return None;
    };
    let block_number = if let Some(matched) = captures.name("block_number") {
        matched
            .as_str()
            .parse::<i32>()
            .unwrap()
            .to_japanese_form()?
    } else {
        return None;
    };
    let rest = if let Some(matched) = captures.name("rest") {
        matched.as_str()
    } else {
        ""
    };
    Some(format!("{}{}丁目{}", town_name, block_number, rest))
}

#[cfg(test)]
mod invalid_town_name_format_test {
    use crate::parser::filter::invalid_town_name_format::InvalidTownNameFormatFilter;
    use crate::parser::filter::Filter;

    #[test]
    fn 有楽町一丁目() {
        let result = InvalidTownNameFormatFilter {}.apply("有楽町1".to_string());
        assert_eq!(result, "有楽町一丁目")
    }

    #[test]
    fn 有楽町一丁目1番() {
        let result = InvalidTownNameFormatFilter {}.apply("有楽町1-1".to_string());
        assert_eq!(result, "有楽町一丁目1")
    }

    #[test]
    fn 有楽町一丁目1番2() {
        let result = InvalidTownNameFormatFilter {}.apply("有楽町1-1-2".to_string());
        assert_eq!(result, "有楽町一丁目1-2")
    }
}
