use crate::entity::Prefecture;

pub fn read_city(input: &str, prefecture: Prefecture) -> Option<(&str, &str)> {}

#[cfg(test)]
mod parser_tests {
    use crate::entity::Prefecture;
    use crate::parser::read_city::read_city;

    #[test]
    fn read_city_成功_京都市山科区() {
        let prefecture = Prefecture {
            name: "京都府".to_string(),
            cities: vec![
                "京都市北区".to_string(),
                "京都市上京区".to_string(),
                "京都市山科区".to_string(),
                "京都市西京区".to_string()],
        };
        let (rest, city) = read_city("京都市山科区椥辻池尻町14-2", prefecture).unwrap();
        assert_eq!(rest, "椥辻池尻町14-2");
        assert_eq!(city, "京都市山科区");
    }

    #[test]
    fn read_city_失敗_市区町村名が誤っている() {
        let prefecture = Prefecture {
            name: "京都府".to_string(),
            cities: vec![
                "京都市北区".to_string(),
                "京都市上京区".to_string(),
                "京都市山科区".to_string(),
                "京都市西京区".to_string()],
        };
        assert_eq!(read_city("港区芝公園4丁目2-8", prefecture), None);
    }
}