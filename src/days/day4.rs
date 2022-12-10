pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day4.txt");

fn get_input(custom_input: Option<String>) -> String {
    return match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, range: &Range) -> bool {
        return self.start <= range.start && self.end >= range.end;
    }

    fn overlaps(&self, range: &Range) -> bool {
        return self.start <= range.end && self.end >= range.start;
    }
}

fn parse_range(raw_range: &str) -> Result<Range, String> {
    let split_result = raw_range.split_once('-');
    if split_result.is_none() {
        return Err(format!("Failed to split by dash: '{}'", raw_range));
    }
    let (raw_start, raw_end) = split_result.unwrap();

    let unchecked_start = raw_start.parse::<u32>();
    let unchecked_end = raw_end.parse::<u32>();

    if unchecked_start.is_err() {
        return Err(format!("Failed to parse start of range '{}'", raw_range));
    } else if unchecked_end.is_err() {
        return Err(format!("Failed to parse end of range '{}'", raw_range));
    }

    let start = unchecked_start.unwrap();
    let end = unchecked_end.unwrap();

    return Ok(Range { start, end });
}

fn get_range_pair(raw_pair: &str) -> Result<(Range, Range), String> {
    let split_result = raw_pair.split_once(',');
    if split_result.is_none() {
        return Err(format!("Failed to split by comma: '{}'", raw_pair));
    }
    let (a, b) = split_result.unwrap();

    let range_a_result = parse_range(a);
    let range_b_result = parse_range(b);

    if range_a_result.is_err() {
        return Err(range_a_result.err().unwrap());
    } else if range_b_result.is_err() {
        return Err(range_b_result.err().unwrap());
    }
    let range_a = range_a_result.unwrap();
    let range_b = range_b_result.unwrap();

    return Ok((range_a, range_b));
}

pub fn solve_part_1(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let mut count = 0;
    for pair in input.lines() {
        let range_pair_result = get_range_pair(pair);
        if range_pair_result.is_err() {
            return Err(range_pair_result.err().unwrap());
        }
        let (range_a, range_b) = range_pair_result.unwrap();

        if range_a.contains(&range_b) || range_b.contains(&range_a) {
            count += 1;
        }
    }
    return Ok(format!("{}", count));
}

pub fn solve_part_2(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let mut count = 0;
    for pair in input.lines() {
        let range_pair_result = get_range_pair(pair);
        if range_pair_result.is_err() {
            return Err(range_pair_result.err().unwrap());
        }
        let (range_a, range_b) = range_pair_result.unwrap();

        if range_a.overlaps(&range_b) {
            count += 1;
        }
    }
    return Ok(format!("{}", count));
}
