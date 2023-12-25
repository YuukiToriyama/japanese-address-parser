pub mod fullwidth_character;

pub trait Filter {
    fn apply(self, input: String) -> String;
}