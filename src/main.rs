fn run_day(day: u8) {
    println!("Running day {}", day);
}

enum CliArgument {
    Day(u8),
    InvalidDay(u8),
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
        day if day.parse::<u8>().is_ok() => parse_day(day.parse::<u8>().unwrap()),
        _ => CliArgument::Unrecognized,
    }
}

static USAGE: &str = "aoc <day> [options]";
static HELP_MESSAGE: &str = "
Run the Advent of Code 2022 solution for the given day.

Arguments:
  day:      The day to run the solution for.          [int]

Options:
  --help:   Show help text.                       [boolean]
";

fn print_error_and_exit(error_message: &str) {
    println!("\x1B[31mError\x1B[0m: {}", error_message);
    std::process::exit(1);
}

fn main() {
    let mut help: bool = false;
    let mut day: Option<u8> = None;

    for arg in std::env::args().skip(1) {
        let parsed_arg = parse_argument(arg.clone());
        match parsed_arg {
            CliArgument::Help => {
                if help {
                    print_error_and_exit("Help flag provided twice");
                }
                help = true;
            }
            CliArgument::Day(day_number) => day = Some(day_number),
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
        run_day(day.unwrap());
    } else {
        print_error_and_exit("Must specify day to run");
    }

}
