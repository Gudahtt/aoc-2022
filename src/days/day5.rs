pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day5.txt");

fn get_input(custom_input: Option<String>) -> String {
    return match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };
}

#[derive(Debug)]
struct Step {
    amount: usize,
    destination: usize,
    source: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum InputState {
    Stacks,
    StackLabels,
    Steps,
}

fn parse_step(line: &str) -> Result<Step, String> {
    let parts: Vec<&str> = line.split(' ').collect();
    if parts.len() != 6 {
        return Err(format!("Invalid step: '{}'", line));
    }

    if parts[1].parse::<usize>().is_err() {
        return Err(format!("Can't parse amount in step '{}'", line));
    } else if parts[3].parse::<usize>().is_err() {
        return Err(format!("Can't parse source in step '{}'", line));
    } else if parts[5].parse::<usize>().is_err() {
        return Err(format!("Can't parse destination in step '{}'", line));
    }
    let amount = parts[1].parse::<usize>().unwrap();
    let source = parts[3].parse::<usize>().unwrap() - 1;
    let destination = parts[5].parse::<usize>().unwrap() - 1;

    return Ok(Step { amount, destination, source })
}

fn parse_input(input: String) -> Result<(Vec<Vec<char>>, Vec<Step>), String> {
    let mut state = InputState::Stacks;
    let mut stack_lines = Vec::new();
    let mut steps = Vec::new();
    let mut num_stacks: usize = 0;
    for line in input.lines() {
        match state {
            InputState::Stacks => {
                if line.get(0..3).is_some() && line.get(0..3).unwrap() == " 1 " {
                    state = InputState::StackLabels;
                    num_stacks = line.trim().split(' ').count();
                } else {
                    stack_lines.push(line.to_string());
                }
            }
            InputState::StackLabels => {
                state = InputState::Steps
            }
            InputState::Steps => {
                let step_result = parse_step(line);
                if step_result.is_err() {
                    return Err(step_result.err().unwrap());
                }
                steps.push(step_result.unwrap());
            }
        }
    }

    if state != InputState::Steps {
        return Err("Incomplete input".to_string());
    }

    let mut stacks = vec![vec![]; num_stacks];
    for line in stack_lines.iter().rev() {
        for stack_index in 0..num_stacks {
            let crate_index = (4 * stack_index) + 1;
            let crate_label_result = line.chars().nth(crate_index);
            if crate_label_result.is_none() {
                break;
            }
            let crate_label = crate_label_result.unwrap();
            if crate_label == ' ' {
                continue;
            } else {
                stacks[stack_index].push(crate_label);
            }
        }
    }

    return Ok((stacks, steps));
}

fn get_top_crates(stacks: Vec<Vec<char>>) -> String {
    let mut top_crates = String::new();
    for mut stack in stacks {
        let top_crate_result = stack.pop();
        if top_crate_result.is_some() {
            top_crates.push(top_crate_result.unwrap())
        }
    }
    return top_crates;
}

pub fn solve_part_1(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let parse_result = parse_input(input);
    if parse_result.is_err() {
        return Err(parse_result.err().unwrap());
    }

    let (mut stacks, steps) = parse_result.unwrap();
    for step in steps {
        for _ in 0..step.amount {
            let crate_to_move_result = stacks[step.source].pop();
            if crate_to_move_result.is_none() {
                return Err(format!("Crate missing for step '{:?}'", step));
            }
            let crate_to_move = crate_to_move_result.unwrap();
            stacks[step.destination].push(crate_to_move);
        }
    }

    let top_crates = get_top_crates(stacks);
    return Ok(top_crates)
}

pub fn solve_part_2(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let parse_result = parse_input(input);
    if parse_result.is_err() {
        return Err(parse_result.err().unwrap());
    }

    let (mut stacks, steps) = parse_result.unwrap();
    for step in steps {
        let mut crates_to_move = Vec::new();
        for _ in 0..step.amount {
            let crate_to_move_result = stacks[step.source].pop();
            if crate_to_move_result.is_none() {
                return Err(format!("Crate missing for step '{:?}'", step));
            }
            let crate_to_move = crate_to_move_result.unwrap();
            crates_to_move.push(crate_to_move);
        }
        crates_to_move.reverse();
        for crate_label in crates_to_move {
            stacks[step.destination].push(crate_label);
        }
    }

    let top_crates = get_top_crates(stacks);
    return Ok(top_crates)
}
