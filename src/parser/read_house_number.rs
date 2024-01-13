#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn read_house_number_with_regex(input: &str) -> Option<(String, String)> {
    let expression = regex::Regex::new(r"(?<house_number>\d+)\D*(?<rest>.*)$").unwrap();
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
    Some((rest.to_string(), format!("{}番", house_number)))
}

#[allow(dead_code)]
#[cfg(target_arch = "wasm32")]
fn read_house_number_with_js_sys_regexp(input: &str) -> Option<(String, String)> {
    let expression = js_sys::RegExp::new(r"(?<house_number>\d+)\D*(?<rest>.*)$", "");
    let captures = expression.exec(input)?;
    let house_number = captures.get(1).as_string()?;
    let rest = captures
        .get(2)
        .as_string()
        .unwrap_or_else(|| "".to_string());
    Some((rest, format!("{}番", house_number)))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::parser::read_house_number::read_house_number_with_regex;

    #[test]
    fn read_house_number_1番() {
        let (rest, house_number) = read_house_number_with_regex("1").unwrap();
        assert_eq!(house_number, "1番");
        assert_eq!(rest, "");
    }

    #[test]
    fn read_house_number_3番2() {
        let (rest, house_number) = read_house_number_with_regex("3-2").unwrap();
        assert_eq!(house_number, "3番");
        assert_eq!(rest, "2");
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use crate::parser::read_house_number::read_house_number_with_js_sys_regexp;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn read_house_number_with_js_sys_regexp_success() {
        let (rest, house_number) = read_house_number_with_js_sys_regexp("1").unwrap();
        assert_eq!(house_number, "1番");
        assert_eq!(rest, "");

        let (rest, house_number) = read_house_number_with_js_sys_regexp("3-2").unwrap();
        assert_eq!(house_number, "3番");
        assert_eq!(rest, "2");
    }
}
