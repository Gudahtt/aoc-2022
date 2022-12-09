
pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day1.txt");

fn get_input(custom_input: Option<String>) -> String {
    return match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };
}

fn get_calorie_groups(input: String) -> Result<Vec<u64>, String> {
    let mut groups = Vec::new();
    let mut current_calories: u64 = 0;
    for line in input.lines() {
        match line {
            "" => {
                groups.push(current_calories);
                current_calories = 0;
            }
            calories if calories.parse::<u64>().is_ok() => {
                current_calories += calories.parse::<u64>().unwrap();
            }
            _ => {
                return Err(format!("Invalid input: '{}'", line));
            }
        }
    }

    return Ok(groups);
}

pub fn solve_part_1(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);
    let elves: Vec<u64>;

    match get_calorie_groups(input) {
        Ok(result) => elves = result,
        Err(error) => return Err(error),
    }

    let max_calories = elves.iter().max();
    return match max_calories {
        Some(result) => Ok(format!("{}", result)),
        None => Err(format!("No elves found"))
    }
}

pub fn solve_part_2(_custom_input: Option<String>) -> Result<String, String> {
    return Err("Not implemented".to_string());
}
