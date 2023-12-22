pub trait JapaneseNumber {
    fn to_japanese_form(self) -> Option<String>;
}

impl JapaneseNumber for i32 {
    fn to_japanese_form(self) -> Option<String> {
        match associate_arabic_number_to_japanese_number(self % 10) {
            Some(result) => Some(result.to_string()),
            None => None,
        }
    }
}

fn associate_arabic_number_to_japanese_number(input: i32) -> Option<&'static str> {
    match input {
        1 => Some("一"),
        2 => Some("二"),
        3 => Some("三"),
        4 => Some("四"),
        5 => Some("五"),
        6 => Some("六"),
        7 => Some("七"),
        8 => Some("八"),
        9 => Some("九"),
        _ => None,
    }
}

#[cfg(test)]
mod japanese_number_converter_tests {
    use crate::util::converter::JapaneseNumber;

    #[test]
    fn to_japanese_form_1桁() {
        assert_eq!(1.to_japanese_form().unwrap(), "一");
        assert_eq!(2.to_japanese_form().unwrap(), "二");
        assert_eq!(3.to_japanese_form().unwrap(), "三");
        assert_eq!(4.to_japanese_form().unwrap(), "四");
        assert_eq!(5.to_japanese_form().unwrap(), "五");
        assert_eq!(6.to_japanese_form().unwrap(), "六");
        assert_eq!(7.to_japanese_form().unwrap(), "七");
        assert_eq!(8.to_japanese_form().unwrap(), "八");
        assert_eq!(9.to_japanese_form().unwrap(), "九");
    }
}