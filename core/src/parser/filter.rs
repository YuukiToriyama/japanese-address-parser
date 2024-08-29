pub trait Filter {
    fn apply(self, input: String) -> String;
}
