use crate::util::converter::JapaneseNumber;

/// 住居表示実施済みの住所でN丁目のNが算用数字の場合に漢数字に書き換えます
pub(crate) fn format_informal_town_name_notation(target: &str) -> Option<String> {
    let (town_name, chome, rest) = if cfg!(target_arch = "wasm32") {
        let captures = js_sys::RegExp::new(
            r"^(\D+)(\d+)[\u002D\u2010\u2011\u2012\u2013\u2014\u2015\u2212\u30FC\uFF0D\uFF70]*(.*)$",
            "",
        ).exec(target)?;
        (
            captures.get(1).as_string()?,
            captures.get(2).as_string()?.parse::<i8>().ok()?,
            captures.get(3).as_string()?,
        )
    } else {
        let captures = regex::Regex::new(
            r"^(?<town_name>\D+)(?<chome>\d+)[\u002D\u2010\u2011\u2012\u2013\u2014\u2015\u2212\u30FC\uFF0D\uFF70]*(?<rest>.*)$",
        ).unwrap().captures(target)?;
        (
            captures.name("town_name")?.as_str().to_string(),
            captures.name("chome")?.as_str().parse::<i8>().ok()?,
            captures.name("rest")?.as_str().to_string(),
        )
    };
    // 帯広市西十九条四十二丁目の42が最大なので、43以上の値の場合はNoneを返すようにする
    if chome > 42 {
        return None;
    }
    Some(format!(
        "{}{}丁目{}",
        town_name,
        chome.to_japanese_form()?,
        rest
    ))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::formatter::informal_town_name_notation::format_informal_town_name_notation;

    #[test]
    fn 町名が検出できない場合() {
        assert_eq!(format_informal_town_name_notation("1-1-1"), None);
    }

    #[test]
    fn 丁目が検出できない場合() {
        assert_eq!(format_informal_town_name_notation("銀座"), None);
    }

    #[test]
    fn 丁目が大きすぎる場合() {
        assert_eq!(
            format_informal_town_name_notation("西十九条42"),
            Some("西十九条四十二丁目".to_string())
        );
        assert_eq!(format_informal_town_name_notation("西十九条43"), None);
    }

    #[test]
    fn 町名以降がない場合() {
        assert_eq!(
            format_informal_town_name_notation("銀座1"),
            Some("銀座一丁目".to_string())
        );
    }

    #[test]
    fn ハイフン以外の文字種が使われている場合() {
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
            let result = format_informal_town_name_notation(input);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), expected);
        }
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use crate::formatter::informal_town_name_notation::format_informal_town_name_notation;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn town_name_not_detected() {
        assert_eq!(format_informal_town_name_notation("1-1-1"), None);
    }

    #[wasm_bindgen_test]
    fn chome_not_detected() {
        assert_eq!(format_informal_town_name_notation("銀座"), None);
    }

    #[wasm_bindgen_test]
    fn chome_is_too_large_number() {
        assert_eq!(
            format_informal_town_name_notation("西十九条42"),
            Some("西十九条四十二丁目".to_string())
        );
        assert_eq!(format_informal_town_name_notation("西十九条43"), None);
    }

    #[wasm_bindgen_test]
    fn rest_is_empty() {
        assert_eq!(
            format_informal_town_name_notation("銀座1"),
            Some("銀座一丁目".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn hyphen_like_characters_are_used() {
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
            let result = format_informal_town_name_notation(input);
            assert!(result.is_some());
            assert_eq!(result.unwrap(), expected);
        }
    }
}
