pub(crate) mod chome_with_arabic_numerals;
pub(crate) mod fullwidth_character;
pub(crate) mod halfwidth_character;
pub(crate) mod house_number;
pub(crate) mod informal_town_name_notation;
pub(crate) mod prepend_aza;
pub(crate) mod prepend_oaza;

pub(crate) type Formatter = fn(&str) -> Option<String>;

pub(crate) fn apply_all(target: &str, formatters: &[Formatter]) -> Option<String> {
    let mut formatted = Some(target.to_string());
    for formatter in formatters {
        formatted = formatted.and_then(|it| formatter(&it))
    }
    formatted
}

#[cfg(test)]
mod tests {
    use crate::formatter::apply_all;
    use crate::formatter::informal_town_name_notation::format_informal_town_name_notation;
    use crate::formatter::prepend_aza::prepend_aza;

    #[test]
    fn formatterが空_元の文字列を返す() {
        let formatted = apply_all("神田神保町3", &[]);
        assert_eq!("神田神保町3", formatted.unwrap());
    }

    #[test]
    fn formatterが1つ_変換後の文字列を返す() {
        let formatted = apply_all("築地1-1-1", &[format_informal_town_name_notation]);
        assert_eq!("築地一丁目1-1", formatted.unwrap());
    }

    #[test]
    fn formatterが2つ_直列に変換処理を行なった文字列を返す() {
        let formatted = apply_all(
            "仲町2-31-8",
            &[format_informal_town_name_notation, prepend_aza],
        );
        assert_eq!("字仲町二丁目31-8", formatted.unwrap());
    }
}
