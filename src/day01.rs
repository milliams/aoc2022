use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};

use crate::read_lines;

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

    if calories_per_elf.len() < number {
        bail!("Not enough Elves");
    }

    Ok(calories_per_elf.iter().rev().take(number).sum())
}

pub fn day1() -> Result<u32> {
    get_max(read_lines!("day01.txt"), 3).context("Getting calorie inventory")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day1() -> Result<()> {
        assert_eq!(
            get_max(
                vec![
                    "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000",
                    "9000", "", "10000"
                ],
                1
            )?,
            24000
        );
        assert_eq!(
            get_max(
                vec![
                    "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000",
                    "9000", "", "10000"
                ],
                3
            )?,
            45000
        );
        assert_eq!(day1()?, 208567);
        Ok(())
    }
}
