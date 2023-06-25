use std::io::{self, Cursor};
use std::io::prelude::*;
use std::fs;

fn main() -> io::Result<()> {
    let file = fs::read("input.txt")?;
    let mut cursor = Cursor::new(file);
    let mut buffer = String::new();
    let mut elfs:  Vec<usize> = vec![0];
    let mut index = 0;
    while cursor.read_line(&mut buffer)? != 0 {
        if buffer.as_str() == "\n" {
            index += 1;
            elfs.push(0);
        } else {
            elfs[index] += buffer.trim().parse::<usize>().unwrap();
        }
        buffer.clear();
    }
    // println!("{:#?}", elfs);
    // part 1
    println!("{}", elfs.iter().max().unwrap());
    Ok(())
}
