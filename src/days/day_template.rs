
pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day1.txt");

fn get_input(custom_input: Option<String>) -> String {
    return match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };
}

pub fn solve_part_1(custom_input: Option<String>) -> Result<String, String> {
    let _input = get_input(custom_input);
    return Err("Not implemented".to_string());
}

pub fn solve_part_2(custom_input: Option<String>) -> Result<String, String> {
    let _input = get_input(custom_input);
    return Err("Not implemented".to_string());
}
