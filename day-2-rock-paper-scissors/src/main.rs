use std::fs;
use std::io::prelude::*;
use std::io::{self, Cursor};

fn main() -> io::Result<()> {
    let file = fs::read("input.txt")?;
    let mut cursor = Cursor::new(file);
    let mut buffer = String::new();
    let mut total_score: usize = 0;
    while cursor.read_line(&mut buffer)? != 0 {
        let mut actions = buffer.trim().split(' ');
        let opponet = actions.next().unwrap();
        let your = actions.next().unwrap();
        total_score += outcome_part2(opponet, your) as usize;
        buffer.clear()
    }
    println!("total_score {total_score}");
    Ok(())
}

fn outcome_part1(opponet: &str, you: &str) -> u8 {
    // a, b, c & x, y, z = rock, paper, scissors
    match opponet {
        "A" => match you {
            "X" => 1 + 3,
            "Y" => 2 + 6,
            "Z" => 3 + 0,
            _ => 0,
        },
        "B" => match you {
            "X" => 1 + 0,
            "Y" => 2 + 3,
            "Z" => 3 + 6,
            _ => 0,
        },
        "C" => match you {
            "X" => 1 + 6,
            "Y" => 2 + 0,
            "Z" => 3 + 3,
            _ => 0,
        },
        _ => 0,
    }
}

fn outcome_part2(opponet: &str, outcome: &str) -> u8 {
    // a, b, c = rock, paper, scissors
    // x, y, z = lose, draw, win
    match outcome {
        // lose
        "X" => match opponet {
            "A" => 0 + 3,
            "B" => 0 + 1,
            "C" => 0 + 2,
            _ => 0,
        },
        "Y" => match opponet {
            "A" => 3 + 1,
            "B" => 3 + 2,
            "C" => 3 + 3,
            _ => 0,
        },
        "Z" => match opponet {
            "A" => 6 + 2,
            "B" => 6 + 3,
            "C" => 6 + 1,
            _ => 0,
        },
        _ => 0,
    }
}
