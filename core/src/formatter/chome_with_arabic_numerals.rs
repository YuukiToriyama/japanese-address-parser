use crate::util::converter::JapaneseNumber;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn format_chome_with_arabic_numerals(target: &str) -> Option<String> {
    let expression = regex::Regex::new(r"\D+(?<block_number>\d+)丁目").unwrap();
    let capture_block_number = expression.captures(target)?.name("block_number")?.as_str();
    let block_number = capture_block_number.parse::<i8>().ok()?;
    Some(target.replacen(
        capture_block_number,
        block_number.to_japanese_form()?.as_str(),
        1,
    ))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn format_chome_with_arabic_numerals(target: &str) -> Option<String> {
    let expression = js_sys::RegExp::new(r"\D+(\d+)丁目", "");
    let capture_block_number = expression.exec(target)?.get(1).as_string()?;
    let block_number = capture_block_number.parse::<i8>().ok()?;
    let block_number_in_japanese_form = block_number.to_japanese_form()?;
    Some(target.replacen(&capture_block_number, &block_number_in_japanese_form, 1))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::formatter::chome_with_arabic_numerals::format_chome_with_arabic_numerals;

    #[test]
    fn filter_with_regex_成功() {
        let result = format_chome_with_arabic_numerals("銀座1丁目");
        assert_eq!(result, Some("銀座一丁目".to_string()));
    }

    #[test]
    fn filter_with_regex_失敗() {
        let result = format_chome_with_arabic_numerals("銀座１丁目");
        assert_eq!(result, None);
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use crate::formatter::chome_with_arabic_numerals::format_chome_with_arabic_numerals;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn filter_with_js_sys_regexp_input_value_will_be_filtered() {
        let result = format_chome_with_arabic_numerals("銀座1丁目");
        assert_eq!(result, Some("銀座一丁目".to_string()));

        let result = format_chome_with_arabic_numerals("銀座1丁目1-1");
        assert_eq!(result, Some("銀座一丁目1-1".to_string()));
    }

    #[wasm_bindgen_test]
    fn filter_with_js_sys_regexp_return_original_value() {
        let result = format_chome_with_arabic_numerals("銀座A丁目");
        assert_eq!(result, Some("銀座A丁目".to_string()));

        let result = format_chome_with_arabic_numerals("銀座2147483648丁目");
        assert_eq!(result, Some("銀座2147483648丁目".to_string()));
    }
}
