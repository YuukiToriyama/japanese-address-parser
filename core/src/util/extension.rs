pub(crate) trait CharExt {
    fn is_variation_selector(&self) -> bool;
}

impl CharExt for char {
    /// 異字体セレクタかどうかを判別します
    fn is_variation_selector(&self) -> bool {
        matches!(self, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}')
    }
}

pub(crate) trait StrExt {
    fn strip_whitespaces(&self) -> String;
    fn strip_variation_selectors(&self) -> String;
}

impl StrExt for str {
    /// 文字列からホワイトスペースを取り除きます
    fn strip_whitespaces(&self) -> String {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }
    /// 文字列から異字体セレクタを取り除きます
    fn strip_variation_selectors(&self) -> String {
        self.chars()
            .filter(|c| !c.is_variation_selector())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::util::extension::{CharExt, StrExt};

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

    #[test]
    fn strip_variation_selectors_逢坂() {
        let normal = "\u{9022}\u{5742}"; // 逢坂
        let variant = "\u{9022}\u{E0101}\u{5742}"; // 逢󠄁坂
        assert_ne!(normal, variant);
        assert_eq!(normal, variant.strip_variation_selectors());
    }

    #[test]
    fn strip_variation_selectors_茨城() {
        let normal = "\u{8328}\u{57CE}";
        let variant = "\u{8328}\u{E0100}\u{57CE}";
        assert_ne!(normal, variant);
        assert_eq!(normal, variant.strip_variation_selectors());
    }

    #[test]
    fn strip_whitespaces() {
        assert_eq!("四谷1丁目".strip_whitespaces(), "四谷1丁目");
        assert_eq!("四谷 1丁目".strip_whitespaces(), "四谷1丁目");
        assert_eq!("四谷  1丁目".strip_whitespaces(), "四谷1丁目");
        assert_eq!("四谷 1 丁 目".strip_whitespaces(), "四谷1丁目");
        assert_eq!("神田３丁目".strip_whitespaces(), "神田３丁目");
        assert_eq!("神田　３丁目".strip_whitespaces(), "神田３丁目");
        assert_eq!("神田　　３丁目".strip_whitespaces(), "神田３丁目");
        assert_eq!("神田　３　丁目".strip_whitespaces(), "神田３丁目");
    }
}
