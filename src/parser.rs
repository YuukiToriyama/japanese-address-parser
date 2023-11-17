pub fn read_prefecture(input: &str) -> Option<(&str, &str)> {}

#[cfg(test)]
mod parser_tests {
    use crate::parser::read_prefecture;

    #[test]
    fn read_prefecture_成功_東京都() {
        let (rest, prefecture) = read_prefecture("東京都港区芝公園4丁目2-8").unwrap();
        assert_eq!(rest, "港区芝公園4丁目2-8");
        assert_eq!(prefecture, "東京都".to_string());
    }

    #[test]
    fn read_prefecture_失敗_都道府県名が誤っている() {
        assert_eq!(read_prefecture("東今日都港区芝公園4丁目2-8"), None);
    }
}
