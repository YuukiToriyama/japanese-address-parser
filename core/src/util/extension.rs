pub(crate) trait CharExt {
    fn is_variation_selector(&self) -> bool;
}

impl CharExt for char {
    /// 異字体セレクタかどうかを判別します
    fn is_variation_selector(&self) -> bool {
        matches!(self, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}')
    }
}

#[cfg(test)]
mod tests {
    use crate::util::extension::CharExt;

    #[test]
    fn is_variation_selector() {
        assert_eq!('あ'.is_variation_selector(), false);
        assert_eq!('亜'.is_variation_selector(), false);

        assert_eq!('\u{FDFF}'.is_variation_selector(), false);
        assert_eq!('\u{FE00}'.is_variation_selector(), true);

        assert_eq!('\u{FE0F}'.is_variation_selector(), true);
        assert_eq!('\u{FE10}'.is_variation_selector(), false);

        assert_eq!('\u{E00FF}'.is_variation_selector(), false);
        assert_eq!('\u{E0100}'.is_variation_selector(), true);

        assert_eq!('\u{E01EF}'.is_variation_selector(), true);
        assert_eq!('\u{E01F0}'.is_variation_selector(), false);
    }
}
