use std::iter::FromIterator;

pub fn reverse(input: &str) -> String {
    let input = String::from(input);

    String::from_iter(input.chars().rev())
}
