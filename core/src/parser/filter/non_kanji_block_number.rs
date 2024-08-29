use crate::parser::filter::Filter;
use crate::util::converter::JapaneseNumber;

pub struct NonKanjiBlockNumberFilter {}

impl Filter for NonKanjiBlockNumberFilter {
    #[cfg(not(target_arch = "wasm32"))]
    fn apply(self, input: String) -> String {
        format_chome_with_arabic_numerals(input)
    }
    #[cfg(target_arch = "wasm32")]
    fn apply(self, input: String) -> String {
        format_chome_with_arabic_numerals(input)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn format_chome_with_arabic_numerals(input: String) -> String {
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
fn format_chome_with_arabic_numerals(input: String) -> String {
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
    use crate::parser::filter::non_kanji_block_number::format_chome_with_arabic_numerals;

    #[test]
    fn filter_with_regex_成功() {
        let result = format_chome_with_arabic_numerals("銀座1丁目".to_string());
        assert_eq!(result, "銀座一丁目");
    }

    #[test]
    fn filter_with_regex_失敗() {
        let result = format_chome_with_arabic_numerals("銀座１丁目".to_string());
        assert_ne!(result, "銀座一丁目");
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use crate::parser::filter::non_kanji_block_number::format_chome_with_arabic_numerals;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn filter_with_js_sys_regexp_input_value_will_be_filtered() {
        let result = format_chome_with_arabic_numerals("銀座1丁目".to_string());
        assert_eq!(result, "銀座一丁目");

        let result = format_chome_with_arabic_numerals("銀座1丁目1-1".to_string());
        assert_eq!(result, "銀座一丁目1-1");
    }

    #[wasm_bindgen_test]
    fn filter_with_js_sys_regexp_return_original_value() {
        let result = format_chome_with_arabic_numerals("銀座A丁目".to_string());
        assert_eq!(result, "銀座A丁目");

        let result = format_chome_with_arabic_numerals("銀座2147483648丁目".to_string());
        assert_eq!(result, "銀座2147483648丁目");
    }
}
