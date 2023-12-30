pub mod fullwidth_character;
pub mod invalid_town_name_format;
pub mod non_kanji_block_number;

pub trait Filter {
    fn apply(self, input: String) -> String;
}
