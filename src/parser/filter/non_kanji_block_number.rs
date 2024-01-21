use crate::parser::filter::Filter;
use crate::util::converter::JapaneseNumber;

pub struct NonKanjiBlockNumberFilter {}

impl Filter for NonKanjiBlockNumberFilter {
    #[cfg(not(target_arch = "wasm32"))]
    fn apply(self, input: String) -> String {
        filter_with_regex(input)
    }
    #[cfg(target_arch = "wasm32")]
    fn apply(self, input: String) -> String {
        filter_with_js_sys_regexp(input)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn filter_with_regex(input: String) -> String {
    let expression = regex::Regex::new(r"\D+(?<block_number>\d+)丁目").unwrap();
    match expression.captures(&input) {
        Some(captures) => {
            let capture_block_number = &captures.name("block_number").unwrap().as_str();
            let block_number = match capture_block_number.parse::<i8>() {
                Ok(x) => x,
                Err(_) => return input,
            };
            input.replacen(
                capture_block_number,
                block_number.to_japanese_form().unwrap().as_str(),
                1,
            )
        }
        None => input,
    }
}

#[cfg(target_arch = "wasm32")]
fn filter_with_js_sys_regexp(input: String) -> String {
    let expression = js_sys::RegExp::new(r"\D+(\d+)丁目", "");
    match expression.exec(&input) {
        Some(result) => {
            let capture_block_number = match result.get(1).as_string() {
                Some(x) => x,
                None => return input,
            };
            let block_number = match capture_block_number.parse::<i8>() {
                Ok(x) => x,
                Err(_) => return input,
            };
            let block_number_in_japanese_form = match block_number.to_japanese_form() {
                Some(x) => x,
                None => return input,
            };
            input.replacen(&capture_block_number, &block_number_in_japanese_form, 1)
        }
        None => input,
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::parser::filter::non_kanji_block_number::filter_with_regex;

    #[test]
    fn filter_with_regex_成功() {
        let result = filter_with_regex("銀座1丁目".to_string());
        assert_eq!(result, "銀座一丁目");
    }

    #[test]
    fn filter_with_regex_失敗() {
        let result = filter_with_regex("銀座１丁目".to_string());
        assert_ne!(result, "銀座一丁目");
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use crate::parser::filter::non_kanji_block_number::filter_with_js_sys_regexp;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn filter_with_js_sys_regexp_input_value_will_be_filtered() {
        let result = filter_with_js_sys_regexp("銀座1丁目".to_string());
        assert_eq!(result, "銀座一丁目");

        let result = filter_with_js_sys_regexp("銀座1丁目1-1".to_string());
        assert_eq!(result, "銀座一丁目1-1");
    }

    #[wasm_bindgen_test]
    fn filter_with_js_sys_regexp_return_original_value() {
        let result = filter_with_js_sys_regexp("銀座A丁目".to_string());
        assert_eq!(result, "銀座A丁目");

        let result = filter_with_js_sys_regexp("銀座2147483648丁目".to_string());
        assert_eq!(result, "銀座2147483648丁目");
    }
}
