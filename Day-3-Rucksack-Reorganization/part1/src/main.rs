use std::fs;
use std::io::prelude::*;
use std::io::{self, Cursor};

fn main() -> io::Result<()> {
    let file = fs::read("input.txt")?;
    let mut cursor = Cursor::new(file);
    let mut buffer = String::new();
    let mut miss_sorted_types: Vec<char> = Vec::new();
    while cursor.read_line(&mut buffer)? != 0 {
        let split_at = buffer.len() / 2;
        let (compartment_1, compartment_2) = buffer.split_at_mut(split_at);
        for i in compartment_1.chars() {
            if compartment_2.find(i).is_some() {
                miss_sorted_types.push(i);
                break;
            }
        }
        buffer.clear();
    }
    let sum = miss_sorted_types
        .iter()
        .fold(0, |acc: usize, x: &char| acc + priority(*x) as usize);
    println!("sum {}", sum);
    Ok(())
}

fn priority(char: char) -> u8 {
    let mut priority: u8 = char.to_uppercase().last().unwrap() as u8 - 0x40;
    if char.is_uppercase() {
        priority += 26;
    }
    return priority;
}
