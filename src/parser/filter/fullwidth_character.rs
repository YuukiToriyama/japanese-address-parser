use crate::parser::filter::Filter;
use itertools::Itertools;

pub struct FullwidthCharacterFilter {}

impl Filter for FullwidthCharacterFilter {
    fn apply(self, input: String) -> String {
        input.chars().map(convert_zenkaku_to_hankaku).join("")
    }
}

fn convert_zenkaku_to_hankaku(c: char) -> char {
    match c {
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
    }
}
