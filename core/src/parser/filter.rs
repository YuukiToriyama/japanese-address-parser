pub mod non_kanji_block_number;

pub trait Filter {
    fn apply(self, input: String) -> String;
}
