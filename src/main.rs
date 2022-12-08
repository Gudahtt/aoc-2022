use std::fs;
mod days;

fn run_day(day: u8, input_path: Option<String>) {
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
    match day {
        1 => days::day1::solve(input),
        _ => print_error_and_exit(format!("Day not found: '{}'", day).as_str()),
    }
}

enum InputValue {
    Path(String),
    None,
}

enum CliArgument {
    Day(u8),
    InvalidDay(u8),
    Input(InputValue),
    Help,
    Unrecognized,
}

fn is_valid_day(day: u8) -> bool {
    return day > 0 && day < 31;
}

fn parse_day(day: u8) -> CliArgument {
    return match day {
        day if is_valid_day(day) => CliArgument::Day(day),
        _ => CliArgument::InvalidDay(day),
    }
}

fn parse_argument(arg: String) -> CliArgument {
    return match arg.as_str() {
        "-h" | "--help" => CliArgument::Help,
        "-i" | "--input" => CliArgument::Input(InputValue::None),
        i if i.starts_with("-i=") => CliArgument::Input(InputValue::Path(i.trim_start_matches("-i=").to_string())),
        i if i.starts_with("--input=") => CliArgument::Input(InputValue::Path(i.trim_start_matches("--input=").to_string())),
        day if day.parse::<u8>().is_ok() => parse_day(day.parse::<u8>().unwrap()),
        _ => CliArgument::Unrecognized,
    }
}

static USAGE: &str = "aoc <day> [options]";
static HELP_MESSAGE: &str = "
Run the Advent of Code 2022 solution for the given day.

If no input is given, the example input will be used.

Arguments:
  day:                     The day to run the solution for.       [int]

Options:
  --help:                  Show help text.                    [boolean]
  --input <path>           Specify which input to use.         [string]
";

fn print_error_and_exit(error_message: &str) {
    println!("\x1B[31mError\x1B[0m: {}", error_message);
    std::process::exit(1);
}

fn main() {
    let mut help: bool = false;
    let mut day: Option<u8> = None;
    let mut next_arg_is_input = false;
    let mut input: Option<String> = None;

    for arg in std::env::args().skip(1) {
        let parsed_arg = parse_argument(arg.clone());
        if next_arg_is_input {
            next_arg_is_input = false;
            input = Some(arg);
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
            CliArgument::Input(input_value) => {
                match input_value {
                    InputValue::Path(path) => {
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
                }
            }
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
        run_day(day.unwrap(), input);
    } else {
        print_error_and_exit("Must specify day to run");
    }

}
