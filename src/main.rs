use std::fs;
mod days;

fn run_day(day: u8, part: Part, input_path: Option<String>) {
    let input: Option<String>;
    match input_path {
        Some(path) => {
            let result = fs::read_to_string(path);
            match result {
                Ok(contents) => input = Some(contents),
                Err(_) => {
                    print_error_and_exit("Failed to load input");
                    input = None;
                }
            }
        }
        None => input = None,
    }
    let result = match day {
        1 => match part {
            Part::Part1 => days::day1::solve_part_1(input),
            Part::Part2 => days::day1::solve_part_2(input),
        },
        2 => match part {
            Part::Part1 => days::day2::solve_part_1(input),
            Part::Part2 => days::day2::solve_part_2(input),
        },
        3 => match part {
            Part::Part1 => days::day3::solve_part_1(input),
            Part::Part2 => days::day3::solve_part_2(input),
        },
        4 => match part {
            Part::Part1 => days::day4::solve_part_1(input),
            Part::Part2 => days::day4::solve_part_2(input),
        },
        5 => match part {
            Part::Part1 => days::day5::solve_part_1(input),
            Part::Part2 => days::day5::solve_part_2(input),
        },
        6 => match part {
            Part::Part1 => days::day6::solve_part_1(input),
            Part::Part2 => days::day6::solve_part_2(input),
        },
        _ => Err(format!("Day not found: '{}'", day)),
    };

    match result {
        Ok(answer) => println!("{}", answer),
        Err(error) => print_error_and_exit(error.as_str()),
    }
}

enum InputValue {
    String(String),
    None,
}

enum CliArgument {
    Day(u8),
    InvalidDay(u8),
    Input(InputValue),
    Part(InputValue),
    Help,
    Unrecognized,
}

enum Part {
    Part1,
    Part2,
}

fn is_valid_day(day: u8) -> bool {
    return day > 0 && day < 31;
}

fn parse_day(day: u8) -> CliArgument {
    return match day {
        day if is_valid_day(day) => CliArgument::Day(day),
        _ => CliArgument::InvalidDay(day),
    };
}

fn parse_argument(arg: String) -> CliArgument {
    return match arg.as_str() {
        "-h" | "--help" => CliArgument::Help,
        "-i" | "--input" => CliArgument::Input(InputValue::None),
        i if i.starts_with("-i=") => {
            CliArgument::Input(InputValue::String(i.trim_start_matches("-i=").to_string()))
        }
        i if i.starts_with("--input=") => CliArgument::Input(InputValue::String(
            i.trim_start_matches("--input=").to_string(),
        )),
        "-p" | "--part" => CliArgument::Part(InputValue::None),
        p if p.starts_with("-p=") => {
            CliArgument::Part(InputValue::String(p.trim_start_matches("-p=").to_string()))
        }
        p if p.starts_with("--part=") => {
            CliArgument::Part(InputValue::String(p.trim_start_matches("--part=").to_string()))
        }
        day if day.parse::<u8>().is_ok() => parse_day(day.parse::<u8>().unwrap()),
        _ => CliArgument::Unrecognized,
    };
}

static USAGE: &str = "aoc <day> [options]";
static HELP_MESSAGE: &str = "
Run the Advent of Code 2022 solution for the given day.

If no input is given, the example input will be used.

Arguments:
  day:                 The day to run the solution for.                  [int]

Options:
  --help:              Show help text.                               [boolean]
  --input <path>       Specify which input to use.                    [string]
  --part <1 or 2>      Specify which part of the puzzle to solve.        [int]
";

fn print_error_and_exit(error_message: &str) {
    println!("\x1B[31mError\x1B[0m: {}", error_message);
    std::process::exit(1);
}

fn main() {
    let mut help: bool = false;
    let mut day: Option<u8> = None;
    let mut next_arg_is_input = false;
    let mut chosen_part: Option<Part> = None;
    let mut next_arg_is_part = false;
    let mut input: Option<String> = None;

    for arg in std::env::args().skip(1) {
        let parsed_arg = parse_argument(arg.clone());
        if next_arg_is_input {
            next_arg_is_input = false;
            input = Some(arg);
            continue;
        } else if next_arg_is_part {
            next_arg_is_part = false;
            match arg.as_str() {
                "1" => chosen_part = Some(Part::Part1),
                "2" => chosen_part = Some(Part::Part2),
                _ => print_error_and_exit(format!("Unrecognized part: '{}'", arg).as_str()),
            }
            continue;
        }
        match parsed_arg {
            CliArgument::Help => {
                if help {
                    print_error_and_exit("Help flag provided twice");
                }
                help = true;
            }
            CliArgument::Day(day_number) => {
                if day.is_some() {
                    print_error_and_exit("Day provided twice");
                }
                day = Some(day_number);
            }
            CliArgument::Input(input_value) => match input_value {
                InputValue::String(path) => {
                    if input.is_some() {
                        print_error_and_exit("Input provided twice");
                    }
                    input = Some(path);
                }
                InputValue::None => {
                    if next_arg_is_input {
                        print_error_and_exit("Input flag provided twice");
                    } else if input.is_some() {
                        print_error_and_exit("Input provided twice");
                    }
                    next_arg_is_input = true
                }
            },
            CliArgument::Part(input_value) => match input_value {
                InputValue::String(part_num) => {
                    if chosen_part.is_some() {
                        print_error_and_exit("Part provided twice");
                    }
                    match part_num.as_str() {
                        "1" => chosen_part = Some(Part::Part1),
                        "2" => chosen_part = Some(Part::Part2),
                        _ => print_error_and_exit(format!("Unrecognized part: '{}'", part_num).as_str()),
                    }
                }
                InputValue::None => {
                    if next_arg_is_part {
                        print_error_and_exit("Part flag provided twice");
                    } else if chosen_part.is_some() {
                        print_error_and_exit("Part provided twice");
                    }
                    next_arg_is_part = true
                }
            },
            CliArgument::InvalidDay(invalid_day) => {
                print_error_and_exit(format!("Unrecognized day: '{}'", invalid_day).as_str());
            }
            CliArgument::Unrecognized => {
                print_error_and_exit(format!("Unrecognized argument: '{}'", arg).as_str());
            }
        }
    }

    if help {
        println!("{}\n{}", USAGE, HELP_MESSAGE);
    } else if day.is_some() {
        let part = match chosen_part {
            Some(p) => p,
            None => Part::Part1,
        };
        run_day(day.unwrap(), part, input);
    } else {
        print_error_and_exit("Must specify day to run");
    }
}
