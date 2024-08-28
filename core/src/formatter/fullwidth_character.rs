/// 文字列中の全角数字を半角数字に修正します
pub(crate) fn format_fullwidth_number(target: &str) -> String {
    target
        .chars()
        .map(|c| match c {
            '０' => '0',
            '１' => '1',
            '２' => '2',
            '３' => '3',
            '４' => '4',
            '５' => '5',
            '６' => '6',
            '７' => '7',
            '８' => '8',
            '９' => '9',
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::formatter::fullwidth_character::format_fullwidth_number;

    #[test]
    fn 全角文字を含む() {
        assert_eq!(format_fullwidth_number("京橋１丁目"), "京橋1丁目");
        assert_eq!(format_fullwidth_number("京橋３丁目１の１"), "京橋3丁目1の1");
    }
}
