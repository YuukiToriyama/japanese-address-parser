use crate::parser::filter::Filter;
use crate::util::converter::JapaneseNumber;
use regex::Regex;

pub struct InvalidTownNameFormatFilter {}

impl Filter for InvalidTownNameFormatFilter {
    fn apply(self, input: String) -> String {
        let (town_name, rest) = if let Some(result) = extract_town_name(&input) {
            result
        } else {
            return input;
        };
        let (house_number, rest) = if let Some(result) = extract_house_number(&rest) {
            result
        } else {
            return format!("{}{}", town_name, rest);
        };
        format!("{}{}{}", town_name, house_number, rest)
    }
}

fn extract_town_name(input: &str) -> Option<(String, String)> {
    let expression = Regex::new(r"^(?<town_name>\D+)(?<block_number>\d+)(?<rest>.*)$").unwrap();
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
    Some((
        format!("{}{}丁目", town_name, block_number),
        rest.to_string(),
    ))
}

fn extract_house_number(input: &str) -> Option<(String, String)> {
    let expression = Regex::new(r"\D+(?<house_number>\d+)\D*(?<rest>.*)$").unwrap();
    let captures = expression.captures(input)?;
    let house_number = if let Some(matched) = captures.name("house_number") {
        matched.as_str()
    } else {
        return None;
    };
    let rest = if let Some(matched) = captures.name("rest") {
        matched.as_str()
    } else {
        ""
    };
    Some((format!("{}番", house_number), rest.to_string()))
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
        assert_eq!(result, "有楽町一丁目1番")
    }

    #[test]
    fn 有楽町一丁目1番2() {
        let result = InvalidTownNameFormatFilter {}.apply("有楽町1-1-2".to_string());
        assert_eq!(result, "有楽町一丁目1番2")
    }
}
