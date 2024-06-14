pub fn trim_city_name(input: &str) -> String {
    match input.chars().position(|c| c == '郡' || c == '市') {
        Some(position) => input.chars().skip(position + 1).collect::<String>(),
        None => input.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::util::trimmer::trim_city_name;

    #[test]
    fn trim_city_name_成功() {
        assert_eq!(trim_city_name("南会津郡下郷町"), "下郷町");
        assert_eq!(trim_city_name("南会津郡只見町"), "只見町");
        assert_eq!(trim_city_name("白河市新白河一丁目"), "新白河一丁目");
    }
}
