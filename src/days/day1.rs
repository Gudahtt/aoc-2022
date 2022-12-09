
pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day1.txt");

pub fn solve(custom_input: Option<String>) -> Result<String, String> {
    let input = match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };

    let mut elves = Vec::new();
    let mut current_elf_calories: u64 = 0;
    for line in input.lines() {
        match line {
            "" => {
                elves.push(current_elf_calories);
                current_elf_calories = 0;
            }
            calories if calories.parse::<u64>().is_ok() => {
                current_elf_calories += calories.parse::<u64>().unwrap();
            }
            _ => {
                return Err(format!("Invalid input: '{}'", line));
            }
        }
    }

    let max_calories = elves.iter().max();
    return match max_calories {
        Some(result) => Ok(format!("{}", result)),
        None => Err(format!("No elves found"))
    }
}
