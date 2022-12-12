use std::collections::{HashMap, HashSet};

pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day8.txt");

fn get_input(custom_input: Option<String>) -> String {
    return match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };
}

fn parse_tree_heights(input: String) -> Result<(u32, u32, HashMap<(u32, u32), u8>), String> {
    let mut tree_heights: HashMap<(u32, u32), u8> = HashMap::new();
    let mut width: Option<u32> = None;
    let mut height: u32 = 0;
    for line in input.lines() {
        if width.is_none() {
            width = Some(line.len() as u32);
        } else if width.unwrap() != line.len() as u32 {
            return Err(format!("Line length invalid, expected '{}', got '{}'", width.unwrap(), line.len()));
        }

        let mut index: u32 = 0;
        for tree_entry in line.chars() {
            if !tree_entry.is_ascii_digit() {
                return Err(format!("Invalid tree entry: '{}'", tree_entry));
            }
            let tree_height: u8 = tree_entry.to_digit(10).unwrap() as u8;
            tree_heights.insert((index, height), tree_height);

            index += 1;
        }
        height += 1;
    }

    if width.is_none() {
        return Err("Input empty".to_string());
    }

    return Ok((width.unwrap(), height, tree_heights));
}

pub fn solve_part_1(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let parse_results = parse_tree_heights(input);
    if parse_results.is_err() {
        return Err(parse_results.err().unwrap());
    }
    let (width, height, tree_heights) = parse_results.unwrap();
    let mut visible_trees: HashSet<(u32, u32)> = HashSet::new();

    for x in 0..width {
        let mut tallest_so_far = None;

        // scan from top to bottom
        for y in 0..height {
            let current_tree_height = tree_heights.get(&(x, y)).unwrap().clone();
            if tallest_so_far.is_none() || tallest_so_far.unwrap() < current_tree_height {
                visible_trees.insert((x, y));
                tallest_so_far = Some(current_tree_height.clone());
            }
            if tallest_so_far.is_some() && tallest_so_far.unwrap() == 9 {
                // early exit
                break;
            }
        }
        // reset
        tallest_so_far = None;

        // scan from bottom to top
        for y in (0..height).rev() {
            let current_tree_height = tree_heights.get(&(x, y)).unwrap().clone();
            if tallest_so_far.is_none() || tallest_so_far.unwrap() < current_tree_height {
                visible_trees.insert((x, y));
                tallest_so_far = Some(current_tree_height.clone());
            }
            if tallest_so_far.is_some() && tallest_so_far.unwrap() == 9 {
                // early exit
                break;
            }
        }
    }

    for y in 0..height {
        let mut tallest_so_far = None;

        // scan from left to right
        for x in 0..width {
            let current_tree_height = tree_heights.get(&(x, y)).unwrap().clone();
            if tallest_so_far.is_none() || tallest_so_far.unwrap() < current_tree_height {
                visible_trees.insert((x, y));
                tallest_so_far = Some(current_tree_height.clone());
            }
            if tallest_so_far.is_some() && tallest_so_far.unwrap() == 9 {
                // early exit
                break;
            }
        }
        // reset
        tallest_so_far = None;

        // scan from right to left
        for x in (0..width).rev() {
            let current_tree_height = tree_heights.get(&(x, y)).unwrap().clone();
            if tallest_so_far.is_none() || tallest_so_far.unwrap() < current_tree_height {
                visible_trees.insert((x, y));
                tallest_so_far = Some(current_tree_height.clone());
            }
            if tallest_so_far.is_some() && tallest_so_far.unwrap() == 9 {
                // early exit
                break;
            }
        }
    }

    return Ok(format!("{}", visible_trees.len()));
}

fn get_scenic_score(tree_heights: &HashMap<(u32, u32), u8>, (width, height): (u32, u32), (tree_x, tree_y): (u32, u32)) -> u64 {
    let tree_height = tree_heights.get(&(tree_x, tree_y)).unwrap().clone();

    let mut right_viewing_distance = 0;
    for x in (tree_x + 1)..width {
        right_viewing_distance += 1;
        let current_tree_height = tree_heights.get(&(x, tree_y)).unwrap().clone();
        if current_tree_height >= tree_height {
            break;
        }
    }

    let mut left_viewing_distance = 0;
    for x in (0..tree_x).rev() {
        left_viewing_distance += 1;
        let current_tree_height = tree_heights.get(&(x, tree_y)).unwrap().clone();
        if current_tree_height >= tree_height {
            break;
        }
    }

    let mut up_viewing_distance = 0;
    for y in (0..tree_y).rev() {
        up_viewing_distance += 1;
        let current_tree_height = tree_heights.get(&(tree_x, y)).unwrap().clone();
        if current_tree_height >= tree_height {
            break;
        }
    }

    let mut down_viewing_distance = 0;
    for y in (tree_y + 1)..height {
        down_viewing_distance += 1;
        let current_tree_height = tree_heights.get(&(tree_x, y)).unwrap().clone();
        if current_tree_height >= tree_height {
            break;
        }
    }

    return left_viewing_distance * right_viewing_distance * up_viewing_distance * down_viewing_distance;
}

pub fn solve_part_2(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let parse_results = parse_tree_heights(input);
    if parse_results.is_err() {
        return Err(parse_results.err().unwrap());
    }
    let (width, height, tree_heights) = parse_results.unwrap();
    if width <= 2 || height <= 2 {
        return Ok("0".to_string());
    }

    let mut highest_scenic_score = None;
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let scenic_score = get_scenic_score(&tree_heights, (width, height), (x, y));
            if highest_scenic_score.is_none() || scenic_score > highest_scenic_score.unwrap() {
                highest_scenic_score = Some(scenic_score);
            }
        }
    }

    return Ok(format!("{}", highest_scenic_score.unwrap()));
}
