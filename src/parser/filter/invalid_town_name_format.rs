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
    let expression = regex::Regex::new(
        r"^(?<town_name>\D+)(?<block_number>\d+)[\u002D\u2010\u2011\u2012\u2013\u2014\u2015\u2212\u30FC\uFF0D\uFF70]*(?<rest>.*)$",
    )
    .unwrap();
    let captures = expression.captures(input)?;
    let town_name = if let Some(matched) = captures.name("town_name") {
        matched.as_str()
    } else {
        return None;
    };
    let block_number = captures.name("block_number")?.as_str().parse::<i8>().ok()?;
    // 帯広市西十九条四十二丁目の42が最大なので、43以上の値の場合はNoneを返すようにする
    if block_number > 42 {
        return None;
    }
    let rest = if let Some(matched) = captures.name("rest") {
        matched.as_str()
    } else {
        ""
    };
    Some(format!(
        "{}{}丁目{}",
        town_name,
        block_number.to_japanese_form()?,
        rest
    ))
}

#[cfg(target_arch = "wasm32")]
fn extract_town_name_with_js_sys_regexp(input: &str) -> Option<String> {
    let expression = js_sys::RegExp::new(
        r"^(\D+)(\d+)[\u002D\u2010\u2011\u2012\u2013\u2014\u2015\u2212\u30FC\uFF0D\uFF70]*(.*)$",
        "",
    );
    let captures = expression.exec(input)?;
    let town_name = captures.get(1).as_string()?;
    let block_number = captures.get(2).as_string()?.parse::<i8>().ok()?;
    // 帯広市西十九条四十二丁目の42が最大なので、43以上の値の場合はNoneを返すようにする
    if block_number > 42 {
        return None;
    }
    let rest = captures
        .get(3)
        .as_string()
        .unwrap_or_else(|| "".to_string());
    Some(format!(
        "{}{}丁目{}",
        town_name,
        block_number.to_japanese_form()?,
        rest
    ))
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

    #[test]
    fn extract_town_name_with_regex_hyphen_like_characters() {
        let test_cases = [
            ("有楽町1-1-1", "有楽町一丁目1-1"),    // U+002D
            ("有楽町1‐1‐1", "有楽町一丁目1‐1"),    // U+2010
            ("有楽町1‑1‑1", "有楽町一丁目1‑1"),    // U+2011
            ("有楽町1‒1‒1", "有楽町一丁目1‒1"),    // U+2012
            ("有楽町1–1–1", "有楽町一丁目1–1"),    // U+2013
            ("有楽町1—1—1", "有楽町一丁目1—1"),    // U+2014
            ("有楽町1―1―1", "有楽町一丁目1―1"),    // U+2015
            ("有楽町1−1−1", "有楽町一丁目1−1"),    // U+2212
            ("有楽町1ー1ー1", "有楽町一丁目1ー1"), // U+30FC
            ("有楽町1－1－1", "有楽町一丁目1－1"), // U+FF0D
            ("有楽町1ｰ1ｰ1", "有楽町一丁目1ｰ1"),    // U+FF70
        ];
        for (input, expected) in test_cases {
            let result = extract_town_name_with_regex(input);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[test]
    fn extract_town_name_with_regex_block_number_boundary_value() {
        let result = extract_town_name_with_regex("西十九条南42");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "西十九条南四十二丁目");
        let result = extract_town_name_with_regex("西十九条南43");
        assert!(result.is_none());
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use crate::parser::filter::invalid_town_name_format::extract_town_name_with_js_sys_regexp;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn extract_town_name_with_js_sys_regexp_success() {
        let result = extract_town_name_with_js_sys_regexp("有楽町1");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "有楽町一丁目");

        let result = extract_town_name_with_js_sys_regexp("有楽町1-1");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "有楽町一丁目1");

        let result = extract_town_name_with_js_sys_regexp("有楽町1-1-2");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "有楽町一丁目1-2");
    }

    #[wasm_bindgen_test]
    fn extract_town_name_with_js_sys_hyphen_like_characters() {
        let test_cases = [
            ("有楽町1-1-1", "有楽町一丁目1-1"),    // U+002D
            ("有楽町1‐1‐1", "有楽町一丁目1‐1"),    // U+2010
            ("有楽町1‑1‑1", "有楽町一丁目1‑1"),    // U+2011
            ("有楽町1‒1‒1", "有楽町一丁目1‒1"),    // U+2012
            ("有楽町1–1–1", "有楽町一丁目1–1"),    // U+2013
            ("有楽町1—1—1", "有楽町一丁目1—1"),    // U+2014
            ("有楽町1―1―1", "有楽町一丁目1―1"),    // U+2015
            ("有楽町1−1−1", "有楽町一丁目1−1"),    // U+2212
            ("有楽町1ー1ー1", "有楽町一丁目1ー1"), // U+30FC
            ("有楽町1－1－1", "有楽町一丁目1－1"), // U+FF0D
            ("有楽町1ｰ1ｰ1", "有楽町一丁目1ｰ1"),    // U+FF70
        ];
        for (input, expected) in test_cases {
            let result = extract_town_name_with_js_sys_regexp(input);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), expected);
        }
    }

    #[wasm_bindgen_test]
    fn extract_town_name_with_js_sys_block_number_boundary_value() {
        let result = extract_town_name_with_js_sys_regexp("西十九条南42");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "西十九条南四十二丁目");
        let result = extract_town_name_with_js_sys_regexp("西十九条南43");
        assert!(result.is_none());
    }

    #[wasm_bindgen_test]
    fn extract_town_name_with_js_sys_regexp_fail() {
        let result = extract_town_name_with_js_sys_regexp("1-1");
        assert!(result.is_none());

        let result = extract_town_name_with_js_sys_regexp("有楽町");
        assert!(result.is_none());
    }
}
