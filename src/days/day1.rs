
pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day1.txt");

pub fn solve(custom_input: Option<String>) {
    let input = match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };

    println!("{}", input);
}
