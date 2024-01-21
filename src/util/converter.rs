pub trait JapaneseNumber {
    fn to_japanese_form(self) -> Option<String>;
}

impl JapaneseNumber for i8 {
    fn to_japanese_form(self) -> Option<String> {
        let first_digit = self % 10;
        let second_digit = (self - self % 10) / 10;
        match (first_digit, second_digit) {
            (0, 0) => None,
            (0, 1) => Some("十".to_string()),
            (0, _) => Some(format!(
                "{}十",
                associate_arabic_number_to_japanese_number(second_digit)?
            )),
            (_, 0) => Some(
                associate_arabic_number_to_japanese_number(first_digit)
                    .unwrap()
                    .to_string(),
            ),
            (_, 1) => Some(format!(
                "十{}",
                associate_arabic_number_to_japanese_number(first_digit).unwrap()
            )),
            (_, _) => Some(format!(
                "{}十{}",
                associate_arabic_number_to_japanese_number(second_digit).unwrap(),
                associate_arabic_number_to_japanese_number(first_digit).unwrap()
            )),
        }
    }
}

fn associate_arabic_number_to_japanese_number(input: i8) -> Option<&'static str> {
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
        assert!(0.to_japanese_form().is_none());
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

    #[test]
    fn to_japanese_form_2桁() {
        assert_eq!(10.to_japanese_form().unwrap(), "十");
        assert_eq!(11.to_japanese_form().unwrap(), "十一");
        assert_eq!(12.to_japanese_form().unwrap(), "十二");
        assert_eq!(13.to_japanese_form().unwrap(), "十三");
        assert_eq!(14.to_japanese_form().unwrap(), "十四");
        assert_eq!(15.to_japanese_form().unwrap(), "十五");
        assert_eq!(16.to_japanese_form().unwrap(), "十六");
        assert_eq!(17.to_japanese_form().unwrap(), "十七");
        assert_eq!(18.to_japanese_form().unwrap(), "十八");
        assert_eq!(19.to_japanese_form().unwrap(), "十九");

        assert_eq!(20.to_japanese_form().unwrap(), "二十");
        assert_eq!(21.to_japanese_form().unwrap(), "二十一");
        assert_eq!(22.to_japanese_form().unwrap(), "二十二");
        assert_eq!(23.to_japanese_form().unwrap(), "二十三");
        assert_eq!(24.to_japanese_form().unwrap(), "二十四");
        assert_eq!(25.to_japanese_form().unwrap(), "二十五");
        assert_eq!(26.to_japanese_form().unwrap(), "二十六");
        assert_eq!(27.to_japanese_form().unwrap(), "二十七");
        assert_eq!(28.to_japanese_form().unwrap(), "二十八");
        assert_eq!(29.to_japanese_form().unwrap(), "二十九");
    }
}
