use std::{collections::HashMap, println, unreachable};

use nom::{
    bytes::complete::tag,
    character::{
        complete::{digit1, newline, space0},
        streaming::space1,
    },
    multi::separated_list1,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

fn main() {
    let file = include_str!("../input.txt");

    let (_, games) = separated_list1(newline, parse_game)(file).unwrap();

    println!(
        "got {} as the sum of valid games for part 1",
        part_1(games.clone())
    );

    println!(
        "got {} as the power of min cubes in games for part 2",
        part_2(games)
    );
}

fn part_1(games: Vec<Game>) -> usize {
    games
        .iter()
        .filter(|game| game.is_valid(12, 13, 14))
        .fold(0, |init, game| init + game.number)
}

fn part_2(games: Vec<Game>) -> usize {
    games.iter().fold(0, |acc, game| {
        let (r, g, b) = game.min_cubes();
        acc + r * g * b
    })
}

#[derive(Debug, Clone)]
struct Game {
    number: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid(&self, max_red: usize, max_green: usize, max_blue: usize) -> bool {
        self.rounds.iter().all(|round| {
            round.cubes.get(&Color::Red).unwrap_or(&0) <= &max_red
                && round.cubes.get(&Color::Green).unwrap_or(&0) <= &max_green
                && round.cubes.get(&Color::Blue).unwrap_or(&0) <= &max_blue
        })
    }

    fn min_cubes(&self) -> (usize, usize, usize) {
        self.rounds
            .iter()
            .map(|round| {
                (
                    round.cubes.get(&Color::Red).unwrap_or(&0),
                    round.cubes.get(&Color::Green).unwrap_or(&0),
                    round.cubes.get(&Color::Blue).unwrap_or(&0),
                )
            })
            .fold((0, 0, 0), |(r, b, g), round| {
                let red = *round.0.max(&r);
                let green = *round.1.max(&g);
                let blue = *round.2.max(&b);
                (red, blue, green)
            })
    }
}

#[test]
fn test_game_is_valid_happy() {
    let cubes1 = vec![(Color::Red, 10), (Color::Blue, 12), (Color::Green, 3)];
    let cubes2 = vec![(Color::Red, 10)];

    let round1 = Round {
        cubes: HashMap::from_iter(cubes1),
    };
    let round2 = Round {
        cubes: HashMap::from_iter(cubes2),
    };

    let game = Game {
        number: 1,
        rounds: vec![round1, round2],
    };

    assert!(game.is_valid(12, 10, 15))
}

#[derive(Debug, Clone)]
struct Round {
    cubes: HashMap<Color, usize>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Color {
    Blue,
    Red,
    Green,
}

// 4 red, 5 blue, 4 green
fn parse_round(input: &str) -> IResult<&str, Round> {
    let mut parser = separated_list1(tag(","), parse_cube);

    let (rest, cubes) = parser(input)?;

    Ok((
        rest,
        Round {
            cubes: HashMap::from_iter(cubes),
        },
    ))
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
    let mut parser = preceded(space0, separated_list1(tag(";"), parse_round));

    parser(input)
}

fn parse_cube(input: &str) -> IResult<&str, (Color, usize)> {
    let colors = nom::branch::alt((tag("blue"), tag("green"), tag("red")));
    let mut parser = pair(delimited(space0, digit1, space1), colors);

    let (rest, (number, color_name)) = parser(input)?;

    let color = match color_name {
        "blue" => Color::Blue,
        "red" => Color::Red,
        "green" => Color::Green,
        _ => unreachable!(),
    };

    Ok((rest, (color, number.parse::<usize>().unwrap())))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let mut parser = tuple((tag("Game"), space1, digit1, tag(":")));

    let (rest, (_, _, number, _)) = parser(input)?;

    let (rest, rounds) = parse_rounds(rest)?;
    Ok((
        rest,
        Game {
            number: number.parse::<usize>().unwrap(),
            rounds,
        },
    ))
}
