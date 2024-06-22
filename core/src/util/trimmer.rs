pub fn trim_city_name(input: &str) -> String {
    // 北海道高市郡と奈良県高市郡は郡名に「市」が入るので特例対応する
    if input.starts_with("余市郡") || input.starts_with("高市郡") {
        return input.chars().skip(3).collect::<String>();
    }

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
        assert_eq!(trim_city_name("余市郡余市町"), "余市町");
        assert_eq!(trim_city_name("余市郡仁木町"), "仁木町");
        assert_eq!(trim_city_name("南会津郡下郷町"), "下郷町");
        assert_eq!(trim_city_name("南会津郡只見町"), "只見町");
        assert_eq!(trim_city_name("白河市新白河一丁目"), "新白河一丁目");
        assert_eq!(trim_city_name("高市郡明日香村"), "明日香村");
    }
}
