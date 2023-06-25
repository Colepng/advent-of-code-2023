use std::io::{self, Cursor};
use std::io::prelude::*;
use std::fs;

fn main() -> io::Result<()> {
    let file = fs::read("input.txt")?;
    let mut cursor = Cursor::new(file);
    let mut buffer = String::new();
    let mut total_score: usize = 0;
    while cursor.read_line(&mut buffer)? != 0 {
        let mut actions = buffer.trim().split(' ');
        let opponet_action = actions.next().unwrap();
        let your_action = actions.next().unwrap();
        total_score += outcome(opponet_action, your_action) as usize;
        buffer.clear()
    }
    println!("total_score {total_score}");
    Ok(())
}

fn outcome(opponet: &str, you: &str) -> u8 {
    // a, b, c & x, y, z = rock, paper, scissors
    match opponet {
        "A" => {
            match you {
                "X" => {
                    1+3
                }
                "Y" => {
                    2+6
                }
                "Z" => {
                    3+0
                }
                _ => {0}
            }
        }
        "B" => {
            match you {
                "X" => {
                    1+0
                }
                "Y" => {
                    2+3
                }
                "Z" => {
                    3+6
                }
                _ => {0}
            }
        }
        "C" => {
            match you {
                "X" => {
                    1+6
                }
                "Y" => {
                    2+0
                }
                "Z" => {
                    3+3
                }
                _ => {0}
            }
        }
        _ => {0}
    }
}
