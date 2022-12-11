
pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day6.txt");

fn get_input(custom_input: Option<String>) -> String {
    return match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };
}

fn find_distinct_characters_index(input: String, characters: usize) -> Result<usize, String> {
    if input.len() < characters {
        return Err("Input too short".to_string());
    }

    let mut marker_index = None;
    let num_windows = input.len() - (characters - 1);
    for window_start_index in 0..num_windows {
        let window = &input[window_start_index..(window_start_index + characters)];
        let mut seen = String::new();

        for char in window.chars() {
            if seen.contains(char) {
                break;
            }
            seen.push(char);
        }
        if seen.len() == characters {
            marker_index = Some(window_start_index + characters);
            break;
        }
    }

    if marker_index.is_none() {
        return Err("Start-of-packet marker not found ".to_string());
    } else {
        return Ok(marker_index.unwrap());
    }
}

pub fn solve_part_1(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let marker_result = find_distinct_characters_index(input, 4);
    if marker_result.is_err() {
        return Err(marker_result.err().unwrap());
    } else {
        return Ok(format!("{}", marker_result.unwrap()));
    }
}

pub fn solve_part_2(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let marker_result = find_distinct_characters_index(input, 14);
    if marker_result.is_err() {
        return Err(marker_result.err().unwrap());
    } else {
        return Ok(format!("{}", marker_result.unwrap()));
    }
}
