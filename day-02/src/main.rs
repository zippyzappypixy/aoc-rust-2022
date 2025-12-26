use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            other => Err(format!("Invalid move token: {}", other)),
        }
    }
}

fn parse_outcome(token: &str) -> Result<Outcome, String> {
    match token {
        "X" => Ok(Outcome::Lose),
        "Y" => Ok(Outcome::Draw),
        "Z" => Ok(Outcome::Win),
        other => Err(format!("Invalid outcome token: {}", other)),
    }
}

impl Move {
    fn shape_score(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

fn round_score(opponent: Move, me: Move) -> u32 {
    let outcome_score = match (me, opponent) {
        (a, b) if a == b => 3, // draw
        (Move::Rock, Move::Scissors)
        | (Move::Scissors, Move::Paper)
        | (Move::Paper, Move::Rock) => 6, // win
        _ => 0,                // loss
    };

    outcome_score + me.shape_score()
}

fn required_move(opponent: Move, desired: Outcome) -> Move {
    match desired {
        Outcome::Draw => opponent,
        Outcome::Win => match opponent {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        Outcome::Lose => match opponent {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
    }
}

fn parse_round(line: &str) -> Result<(Move, Move), String> {
    let mut parts = line.split_whitespace();
    let opponent = parts
        .next()
        .ok_or_else(|| format!("Missing opponent move in line: {}", line))?
        .parse::<Move>()?;
    let me = parts
        .next()
        .ok_or_else(|| format!("Missing my move in line: {}", line))?
        .parse::<Move>()?;
    Ok((opponent, me))
}

fn main() {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let contents = fs::read_to_string(&input_path).unwrap_or_else(|err| {
        eprintln!("Failed to read {}: {}", input_path.display(), err);
        std::process::exit(1);
    });

    let total_score: u32 = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (opponent, me) = parse_round(line).unwrap_or_else(|err| {
                eprintln!("{}", err);
                std::process::exit(1);
            });
            round_score(opponent, me)
        })
        .sum();

    let total_score_part2: u32 = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let opponent = parts
                .next()
                .ok_or_else(|| format!("Missing opponent move in line: {}", line))
                .and_then(|tok| tok.parse::<Move>())
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
            let desired = parts
                .next()
                .ok_or_else(|| format!("Missing desired outcome in line: {}", line))
                .and_then(parse_outcome)
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
            let my_move = required_move(opponent, desired);
            round_score(opponent, my_move)
        })
        .sum();

    println!("Total score (Part 1 logic): {}", total_score);
    println!("Total score (Part 2 logic): {}", total_score_part2);
}
