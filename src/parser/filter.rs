pub mod fullwidth_character;
pub mod non_kanji_block_number;
pub mod invalid_town_name_format;

pub trait Filter {
    fn apply(self, input: String) -> String;
}
