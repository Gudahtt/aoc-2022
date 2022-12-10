pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day3.txt");

fn get_input(custom_input: Option<String>) -> String {
    return match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };
}

fn get_priority(item: char) -> Result<u32, String> {
    return match item {
        c if c.is_ascii_lowercase() => Ok(1 + item as u32 - 'a' as u32),
        c if c.is_ascii_uppercase() => Ok(27 + item as u32 - 'A' as u32),
        _ => Err(format!("Invalid item: '{}'", item)),
    };
}

fn get_unique_common_characters(a: &str, b: &str) -> String {
    let mut results = String::new();
    for item in a.chars() {
        if b.contains(item) && !results.contains(item) {
            results.push(item);
        }
    }
    return results;
}

pub fn solve_part_1(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let mut score: u32 = 0;
    for line in input.lines() {
        let length = line.len();
        if length % 2 != 0 {
            return Err(format!("Line cannot be evenly split: '{}'", line))
        }
        let (first, second) = line.split_at(length / 2);

        let common = get_unique_common_characters(first, second);

        for item in common.chars() {
            match get_priority(item) {
                Ok(item_score) => score += item_score,
                Err(error) => return Err(error),
            }
        }
    }

    return Ok(format!("{}", score));
}

pub fn solve_part_2(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let mut badges = String::new();
    let mut group = vec![];
    for line in input.lines() {
        group.push(line);

        if group.len() < 3 {
            continue;
        }

        let initial_common = get_unique_common_characters(group[0], group[1]);
        let common = get_unique_common_characters(&initial_common, group[2]);

        if common.len() == 0 {
            return Err(format!("No badge found for group starting with '{}'", group[0]));
        } else if common.len() > 1 {
            return Err(format!("Multiple badges found ('{}') in group starting with '{}'", common, group[0]));
        }

        badges.push_str(common.as_str());
        group.clear();
    }

    if group.len() > 0 {
        return Err(format!("Incomplete group starting with '{}'", group[0]))
    }

    let mut score: u32 = 0;
    for item in badges.chars() {
        match get_priority(item) {
            Ok(item_score) => score += item_score,
            Err(error) => return Err(error),
        }
    }

    return Ok(format!("{}", score));
}
