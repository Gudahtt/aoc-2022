use std::collections::HashMap;

pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day7.txt");

fn get_input(custom_input: Option<String>) -> String {
    return match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };
}

enum Destination {
    Directory(String),
    Root,
    Up,
}

enum Command {
    ChangeDirectory(Destination),
    List,
}

fn parse_command(command: &str) -> Result<Command, String> {
    return match command {
        "ls" => Ok(Command::List),
        "cd .." => Ok(Command::ChangeDirectory(Destination::Up)),
        "cd /" => Ok(Command::ChangeDirectory(Destination::Root)),
        c if c.starts_with("cd ")
            && c.len() > 3
            && c[3..c.len()]
                .matches(char::is_alphanumeric)
                .collect::<String>()
                .len()
                == c[3..c.len()].len() =>
        {
            Ok(Command::ChangeDirectory(Destination::Directory(
                command[3..command.len()].to_string(),
            )))
        }
        _ => Err(format!("Unrecognized command: '{}'", command)),
    };
}

fn get_working_directory_path(working_directory: &Vec<String>) -> String {
    let mut working_directory_path = "/".to_string();
    working_directory_path.push_str(working_directory.join("/").as_str());
    return working_directory_path;
}

fn parse_input(
    input: String,
) -> Result<(HashMap<String, Vec<String>>, HashMap<String, u64>), String> {
    let mut working_directory = vec![];
    let mut list_output_expected = false;
    let mut directory_contents: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_sizes: HashMap<String, u64> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("$ ") {
            list_output_expected = false;
            let raw_command = &line[2..line.len()];
            let command_result = parse_command(raw_command);
            if command_result.is_err() {
                return Err(format!("Unable to parse command '{}'", raw_command));
            }
            let command = command_result.unwrap();

            match command {
                Command::List => {
                    list_output_expected = true;
                }
                Command::ChangeDirectory(destination) => match destination {
                    Destination::Directory(name) => {
                        working_directory.push(name);
                    }
                    Destination::Root => {
                        working_directory.clear();
                    }
                    Destination::Up => {
                        if working_directory.len() > 0 {
                            working_directory.pop();
                        } else {
                            return Err(format!(
                                "Unable to process command '{}', can't go up past root.",
                                line
                            ));
                        }
                    }
                },
            }
        } else if list_output_expected {
            if line.starts_with("dir ") {
                let working_directory_path = get_working_directory_path(&working_directory);

                let directory_name = &line[4..line.len()];
                if directory_name.len() == 0
                    || directory_name
                        .matches(char::is_alphanumeric)
                        .collect::<String>()
                        .len()
                        != directory_name.len()
                {
                    return Err(format!("Invalid directory name in ls entry: '{}'", line));
                }
                let mut directory_path = working_directory_path.clone();
                if !directory_path.ends_with("/") {
                    directory_path.push_str("/");
                }
                directory_path.push_str(directory_name);

                directory_contents
                    .entry(working_directory_path)
                    .and_modify(|contents| contents.push(directory_path.clone()))
                    .or_insert(vec![directory_path]);
            } else {
                let split_result = line.split_once(' ');
                if split_result.is_none() {
                    return Err(format!("Unrecognized directory entry: '{}'", line));
                }
                let (raw_size, name) = split_result.unwrap();
                let parse_result = raw_size.parse::<u64>();
                if parse_result.is_err() {
                    return Err(format!("Failed to parse size as integer: '{}'", raw_size));
                }
                let size = parse_result.unwrap();

                let working_directory_path = get_working_directory_path(&working_directory);
                let mut file_path = working_directory_path.clone();
                if !file_path.ends_with("/") {
                    file_path.push_str("/");
                }
                file_path.push_str(name);

                directory_contents
                    .entry(working_directory_path)
                    .and_modify(|contents| contents.push(file_path.clone()))
                    .or_insert(vec![file_path.clone()]);

                if file_sizes.contains_key(&file_path) {
                    if file_sizes[&file_path] != size {
                        return Err(format!(
                            "Duplicate file entry found with wrong size: '{}', original size: '{}'",
                            line, file_sizes[&file_path]
                        ));
                    }
                } else {
                    file_sizes.insert(file_path, size);
                }
            }
        } else {
            return Err(format!("Unexpected line '{}'", line));
        }
    }

    return Ok((directory_contents, file_sizes));
}

fn get_directory_sizes(directory_contents: HashMap<String, Vec<String>>, file_sizes: HashMap<String, u64>) -> Result<HashMap<String, u64>, String> {
    let mut directory_sizes: HashMap<String, u64> = HashMap::new();
    let mut directories: Vec<&String> = directory_contents.keys().collect();
    directories.sort_by_cached_key(|name| name.len());
    directories.reverse();

    for directory in directories {
        let mut size: u64 = 0;
        for entry in &directory_contents[directory] {
            if file_sizes.contains_key(entry) {
                size += file_sizes[entry];
            } else if directory_sizes.contains_key(entry) {
                size += directory_sizes[entry];
            } else {
                return Err(format!(
                    "Unable to find entry '{}' for directory '{}'",
                    entry, directory
                ));
            }
        }
        directory_sizes.insert(directory.clone(), size);
    }
    return Ok(directory_sizes);
}

pub fn solve_part_1(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let parse_result = parse_input(input);
    if parse_result.is_err() {
        return Err(parse_result.err().unwrap());
    }
    let (directory_contents, file_sizes) = parse_result.unwrap();

    let size_results = get_directory_sizes(directory_contents, file_sizes);
    if size_results.is_err() {
        return Err(size_results.err().unwrap());
    }
    let directory_sizes = size_results.unwrap();

    let mut sizes: Vec<u64> = directory_sizes.into_iter().map(|(_, v)| v).collect();
    sizes.retain(|size| *size <= 100_000);
    let sum: u64 = sizes.iter().sum();

    return Ok(format!("{}", sum))
}

pub fn solve_part_2(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let parse_result = parse_input(input);
    if parse_result.is_err() {
        return Err(parse_result.err().unwrap());
    }
    let (directory_contents, file_sizes) = parse_result.unwrap();

    let size_results = get_directory_sizes(directory_contents, file_sizes);
    if size_results.is_err() {
        return Err(size_results.err().unwrap());
    }
    let directory_sizes = size_results.unwrap();

    let total_filesystem_size: u64 = 70_000_000;
    let required_unused_space: u64 = 30_000_000;
    let current_unused_space = total_filesystem_size - directory_sizes["/"];

    if current_unused_space >= required_unused_space {
        return Err("Already enough unused space present".to_string());
    }

    let additional_required_space = required_unused_space - current_unused_space;

    let mut sizes: Vec<u64> = directory_sizes.into_iter().map(|(_, v)| v).collect();
    sizes.retain(|size| *size >= additional_required_space);
    sizes.sort();

    if sizes.len() == 0 {
        return Err("No directories found that were large enough to fee up required space".to_string());
    }

    return Ok(format!("{}", sizes[0]))
}
