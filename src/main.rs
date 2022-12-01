use std::fs::File;
use std::io::{BufReader, BufRead};

use anyhow::{Context, Result, bail};

macro_rules! read_lines {
    ($expression:expr) => {
        BufReader::new(File::open($expression)?).lines().filter(|l| l.is_ok()).map(|l| l.unwrap())
    };
}

fn get_max<I>(lines: I, number: usize) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut calories_per_elf = vec![];
    let mut this_elf_calories = 0;

    for line in lines {
        let line = line.as_ref();
        if line.is_empty() {
            calories_per_elf.push(this_elf_calories);
            this_elf_calories = 0;
        } else {
            let calories: u32 = line.parse()?;
            this_elf_calories += calories;
        }
    }
    calories_per_elf.push(this_elf_calories);

    calories_per_elf.sort();

    if calories_per_elf.len() >= number {
        bail!("Not enough Elves");
    }

    Ok(calories_per_elf.iter().rev().take(number).sum())
}

fn day1() -> Result<u32> {
    get_max(read_lines!("day1.txt"), 3)
}

fn main() -> Result<()> {
    let max_calories = day1()?;
    println!("{}", max_calories);

    Ok(())
}
