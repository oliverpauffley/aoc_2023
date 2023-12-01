use regex::Regex;
use std::{error::Error, str::FromStr};

fn main() {
    let file = include_str!("../input.txt");

    let part_1 = part_1(file);

    let part_2 = part_2(file);

    println!("got {part_1} for part 1");
    println!("got {part_2} for part 2")
}

fn part_1(input: &str) -> isize {
    let calibration_values = input.lines().map(|line| {
        let mut nums = line.chars().filter(|&c| c.is_ascii_digit());
        let first = nums.next().unwrap();
        let last = nums.last().unwrap_or(first);
        format!("{}{}", first, last)
    });
    calibration_values.fold(0, |init, x| x.parse::<isize>().unwrap() + init)
}

fn part_2(input: &str) -> isize {
    let calibration_values = input.lines().map(|line| {
        let (first, last) = find_digits(line);
        format!("{first}{last}").parse::<isize>().unwrap()
    });
    calibration_values.sum()
}

fn find_digits(line: &str) -> (isize, isize) {
    let reg = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d{1}").unwrap();
    let reg_reverse = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d{1}").unwrap();

    let reverse_line = line.chars().rev().collect::<String>();
    let first = reg.find(line).unwrap();
    let last_reverse = reg_reverse.find(&reverse_line).unwrap();

    let last = last_reverse.as_str().chars().rev().collect::<String>();
    let first = parse_match(first.as_str());
    let last = parse_match(last.as_str());
    (first, last)
}

#[test]
fn test_find_digits() {
    let tests = [
        ("one18", (1, 8)),
        ("9", (9, 9)),
        ("88", (8, 8)),
        ("eight", (8, 8)),
        ("eighteight", (8, 8)),
        ("ninenine", (9, 9)),
        ("7seven", (7, 7)),
        ("9nine", (9, 9)),
        ("9nine", (9, 9)),
        ("33", (3, 3)),
        ("two 12345 two", (2, 2)),
        ("8 12345 one", (8, 1)),
        ("8a 12345 bone", (8, 1)),
        ("foureightmppchbgz8lqbzqbjztwo7cksqxns", (4, 7)),
        ("stwone1oneninevcrfzpfourfivetwone", (2, 1)),
        ("twone", (2, 1)),
    ];
    for t in tests {
        assert_eq!(find_digits(t.0), t.1)
    }
}

#[test]
fn test_part_2() {
    let input = "11abs
2aba12
12aadseven
three
four
five
six
seven
eight
nine
";
    assert_eq!(part_2(input), 512)
}

fn parse_match(input: &str) -> isize {
    if input.len() == 1 {
        return input.parse::<isize>().unwrap();
    }

    NumberAsString::from_str(input).unwrap().into()
}

enum NumberAsString {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<NumberAsString> for isize {
    fn from(value: NumberAsString) -> Self {
        match value {
            NumberAsString::One => 1,
            NumberAsString::Two => 2,
            NumberAsString::Three => 3,
            NumberAsString::Four => 4,
            NumberAsString::Five => 5,
            NumberAsString::Six => 6,
            NumberAsString::Seven => 7,
            NumberAsString::Eight => 8,
            NumberAsString::Nine => 9,
        }
    }
}

impl FromStr for NumberAsString {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "one" => Ok(NumberAsString::One),
            "two" => Ok(NumberAsString::Two),
            "three" => Ok(NumberAsString::Three),
            "four" => Ok(NumberAsString::Four),
            "five" => Ok(NumberAsString::Five),
            "six" => Ok(NumberAsString::Six),
            "seven" => Ok(NumberAsString::Seven),
            "eight" => Ok(NumberAsString::Eight),
            "nine" => Ok(NumberAsString::Nine),
            _ => Err("not a number".into()),
        }
    }
}
