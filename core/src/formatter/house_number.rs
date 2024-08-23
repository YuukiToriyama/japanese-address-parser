#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn format_house_number(input: &str) -> Result<String, &'static str> {
    let captures = regex::Regex::new(r"(?<block_number>\d+)\D+(?<house_number>\d+)(?<rest>.*)$")
        .unwrap()
        .captures(input)
        .ok_or("マッチするものがありませんでした")?;
    let block_number = captures
        .name("block_number")
        .ok_or("街区符号を検出できませんでした")?;
    let house_number = captures
        .name("house_number")
        .ok_or("住居番号を検出できませんでした")?;
    let rest = match captures.name("rest") {
        Some(matched) => matched.as_str(),
        None => "",
    };
    Ok(format!(
        "{}番{}号{}",
        block_number.as_str(),
        house_number.as_str(),
        rest
    ))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn format_house_number(input: &str) -> Result<String, &'static str> {
    let captures = js_sys::RegExp::new(
        r"(?<block_number>\d+)\D+(?<house_number>\d+)(?<rest>.*)$",
        "",
    )
    .exec(input)
    .ok_or("マッチするものがありませんでした")?;
    let block_number = captures
        .get(1)
        .as_string()
        .ok_or("街区符号を検出できませんでした")?;
    let house_number = captures
        .get(2)
        .as_string()
        .ok_or("住居番号を検出できませんでした")?;
    let rest = captures
        .get(3)
        .as_string()
        .unwrap_or_else(|| "".to_string());
    Ok(format!("{}番{}号{}", block_number, house_number, rest))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::formatter::house_number::format_house_number;

    #[test]
    fn format_house_number_1番1号() {
        let result = format_house_number("1-1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1番1号");
    }

    #[test]
    fn format_house_number_3番2号レジデンシャルマンション101号室() {
        let result = format_house_number("3-2レジデンシャルマンション101号室");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3番2号レジデンシャルマンション101号室");
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use crate::formatter::house_number::format_house_number;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn format_house_number_success() {
        let result = format_house_number("1-1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1番1号");

        let result = format_house_number("3-2レジデンシャルマンション101号室");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3番2号レジデンシャルマンション101号室");
    }
}
