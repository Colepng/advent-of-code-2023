#![feature(iter_array_chunks)]
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let files = fs::read("input.txt")?;
    let lines = files.lines();
    let mut badges: Vec<char> = Vec::new();
    for i in lines.array_chunks::<3>() {
        let len = i[0].as_ref().unwrap().len();
        for j in 0..len {
            let rucksack_1 = i[0].as_ref().unwrap();
            let rucksack_2 = i[1].as_ref().unwrap();
            let rucksack_3 = i[2].as_ref().unwrap();
            let element = rucksack_1.chars().nth(j).unwrap();
            if rucksack_1.contains(element)
                && rucksack_2.contains(element)
                && rucksack_3.contains(element)
            {
                badges.push(element);
                break;
            }
        }
    }

    let sum = badges.iter().fold(0, |acc: usize, x: &char| acc + priority(*x) as usize);
    println!("sum, {sum}");

    Ok(())
}

fn priority(char: char) -> u8 {
    let mut priority: u8 = char.to_uppercase().last().unwrap() as u8 - 0x40;
    if char.is_uppercase() {
        priority += 26;
    }
    priority
}
