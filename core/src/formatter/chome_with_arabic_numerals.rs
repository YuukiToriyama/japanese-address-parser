use crate::util::converter::JapaneseNumber;

pub(crate) fn format_chome_with_arabic_numerals(target: &str) -> Option<String> {
    let chome = if cfg!(target_arch = "wasm32") {
        js_sys::RegExp::new(r"\D+(\d+)丁目", "")
            .exec(target)?
            .get(1)
            .as_string()?
    } else {
        regex::Regex::new(r"\D+(?<chome>\d+)丁目")
            .unwrap()
            .captures(target)?
            .name("chome")?
            .as_str()
            .to_string()
    };
    let chome_int = chome.parse::<i8>().ok()?;
    Some(target.replacen(&chome, chome_int.to_japanese_form()?.as_str(), 1))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::formatter::chome_with_arabic_numerals::format_chome_with_arabic_numerals;

    #[test]
    fn 丁目を検出できない場合() {
        assert_eq!(format_chome_with_arabic_numerals("a丁目"), None);
    }

    #[test]
    fn 丁目をi8に変換できない場合() {
        assert_eq!(
            format_chome_with_arabic_numerals("銀座127丁目"),
            Some("銀座百二十七丁目".to_string())
        );
        assert_eq!(format_chome_with_arabic_numerals("銀座128丁目"), None);
    }

    #[test]
    fn 成功() {
        assert_eq!(
            format_chome_with_arabic_numerals("銀座1丁目"),
            Some("銀座一丁目".to_string())
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use crate::formatter::chome_with_arabic_numerals::format_chome_with_arabic_numerals;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn chome_not_detected() {
        assert_eq!(format_chome_with_arabic_numerals("a丁目"), None);
    }

    #[wasm_bindgen_test]
    fn failed_to_convert_chome_into_i8() {
        assert_eq!(
            format_chome_with_arabic_numerals("銀座127丁目"),
            Some("銀座百二十七丁目".to_string())
        );
        assert_eq!(format_chome_with_arabic_numerals("銀座128丁目"), None);
    }

    #[wasm_bindgen_test]
    fn success() {
        assert_eq!(
            format_chome_with_arabic_numerals("銀座1丁目"),
            Some("銀座一丁目".to_string())
        );
    }
}
