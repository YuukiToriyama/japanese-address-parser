use crate::parser::filter::Filter;
use crate::util::converter::JapaneseNumber;

pub struct InvalidTownNameFormatFilter {}

impl Filter for InvalidTownNameFormatFilter {
    fn apply(self, input: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod invalid_town_name_format_test {
    use crate::parser::filter::Filter;
    use crate::parser::filter::invalid_town_name_format::InvalidTownNameFormatFilter;

    #[test]
    fn 有楽町一丁目() {
        let result = InvalidTownNameFormatFilter {}.apply("有楽町1".to_string());
        assert_eq!(result, "有楽町一丁目")
    }

    #[test]
    fn 有楽町一丁目1番() {
        let result = InvalidTownNameFormatFilter {}.apply("有楽町1-1".to_string());
        assert_eq!(result, "有楽町一丁目1番")
    }

    #[test]
    fn 有楽町一丁目1番2() {
        let result = InvalidTownNameFormatFilter {}.apply("有楽町1-1-2".to_string());
        assert_eq!(result, "有楽町一丁目1番2")
    }
}