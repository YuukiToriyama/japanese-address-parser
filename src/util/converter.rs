pub trait JapaneseNumber {
    fn to_japanese_form(self) -> Option<&'static str>;
}
