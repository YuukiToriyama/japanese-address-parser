use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::Parser;

const PREFECTURE_NAME_LIST: [&str; 47] = [
    "北海道",
    "青森県",
    "岩手県",
    "宮城県",
    "秋田県",
    "山形県",
    "福島県",
    "茨城県",
    "栃木県",
    "群馬県",
    "埼玉県",
    "千葉県",
    "東京都",
    "神奈川県",
    "新潟県",
    "富山県",
    "石川県",
    "福井県",
    "山梨県",
    "長野県",
    "岐阜県",
    "静岡県",
    "愛知県",
    "三重県",
    "滋賀県",
    "京都府",
    "大阪府",
    "兵庫県",
    "奈良県",
    "和歌山県",
    "鳥取県",
    "島根県",
    "岡山県",
    "広島県",
    "山口県",
    "徳島県",
    "香川県",
    "愛媛県",
    "高知県",
    "福岡県",
    "佐賀県",
    "長崎県",
    "熊本県",
    "大分県",
    "宮崎県",
    "鹿児島県",
    "沖縄県",
];

pub fn read_prefecture(input: &str) -> Option<(&str, &str)> {
    for prefecture_name in PREFECTURE_NAME_LIST {
        if let Ok(result) = tag::<&str, &str, VerboseError<&str>>(prefecture_name).parse(input) {
            return Some(result);
        }
    }
    None
}

#[cfg(test)]
mod parser_tests {
    use crate::parser::read_prefecture::read_prefecture;

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
