use crate::parser::filter::Filter;
use crate::util::converter::JapaneseNumber;

pub struct InvalidTownNameFormatFilter {}

impl Filter for InvalidTownNameFormatFilter {
    #[cfg(not(target_arch = "wasm32"))]
    fn apply(self, input: String) -> String {
        extract_town_name_with_regex(&input).unwrap_or(input)
    }
    #[cfg(target_arch = "wasm32")]
    fn apply(self, input: String) -> String {
        extract_town_name_with_js_sys_regexp(&input).unwrap_or(input)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn extract_town_name_with_regex(input: &str) -> Option<String> {
    let expression =
        regex::Regex::new(r"^(?<town_name>\D+)(?<block_number>\d+)[-ー－]*(?<rest>.*)$").unwrap();
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

#[cfg(target_arch = "wasm32")]
fn extract_town_name_with_js_sys_regexp(input: &str) -> Option<String> {
    let expression = js_sys::RegExp::new(
        r"^(?<town_name>\D+)(?<block_number>\d+)[-ー－]*(?<rest>.*)$",
        "",
    );
    let captures = expression.exec(input)?;
    let town_name = match captures.get(1).as_string() {
        Some(matched) => matched,
        None => return None,
    };
    let block_number = match captures.get(2).as_string() {
        Some(matched) => matched.parse::<i32>().unwrap().to_japanese_form()?,
        None => return None,
    };
    let rest = captures
        .get(3)
        .as_string()
        .unwrap_or_else(|| "".to_string());
    Some(format!("{}{}丁目{}", town_name, block_number, rest))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::parser::filter::invalid_town_name_format::extract_town_name_with_regex;

    #[test]
    fn extract_town_name_with_regex_success() {
        let result = extract_town_name_with_regex("有楽町1");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "有楽町一丁目");

        let result = extract_town_name_with_regex("有楽町1-1");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "有楽町一丁目1");

        let result = extract_town_name_with_regex("有楽町1-1-2");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "有楽町一丁目1-2");
    }
}
