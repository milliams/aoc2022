use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

use anyhow::{Context, Result, bail};
use itertools::Itertools;

macro_rules! read_lines {
    ($expression:expr) => {
        BufReader::new(File::open($expression)?).lines().filter(|l| l.is_ok()).map(|l| l.unwrap())
    };
}

enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Lose,
    Draw,
    Win,
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

    if calories_per_elf.len() < number {
        bail!("Not enough Elves");
    }

    Ok(calories_per_elf.iter().rev().take(number).sum())
}

fn calculate_rps_score<I>(lines: I) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut round_scores = vec![];
    for line in lines {
        let line = line.as_ref();
        if line.len() != 3 {
            bail!("Line is wrong length");
        }
        let other_move = match line.chars().nth(0).context("No zeroth char")? {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            _ => bail!("Invalid move")
        };
        let desired_result = match line.chars().nth(2).context("No second char")? {
            'X' => GameResult::Lose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => bail!("Invalid strategy")
        };

        let play = match (&other_move, &desired_result) {
            (RPS::Rock, GameResult::Lose) => RPS::Scissors,
            (RPS::Rock, GameResult::Draw) => RPS::Rock,
            (RPS::Rock, GameResult::Win) => RPS::Paper,
            (RPS::Paper, GameResult::Lose) => RPS::Rock,
            (RPS::Paper, GameResult::Draw) => RPS::Paper,
            (RPS::Paper, GameResult::Win) => RPS::Scissors,
            (RPS::Scissors, GameResult::Lose) => RPS::Paper,
            (RPS::Scissors, GameResult::Draw) => RPS::Scissors,
            (RPS::Scissors, GameResult::Win) => RPS::Rock,
        };

        let shape_score = match play {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        let play_score = match desired_result {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        };

        round_scores.push(shape_score + play_score);
    }
    Ok(round_scores.iter().sum())
}

fn calculate_backpack_score<I>(lines: I) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut item_priorities = vec![];
    for line in lines {
        let line = line.as_ref();
        let compartment_size = line.len() / 2;
        let left: HashSet<char> = HashSet::from_iter(line[..compartment_size].chars());
        let right: HashSet<char> = HashSet::from_iter(line[compartment_size..].chars());
        let overlap = left.intersection(&right).take(1).nth(0).context("No misplaced item")?;
        let priority = match overlap {
            'A'..='Z' => *overlap as u32 - 38,  // A-Z = 27-52
            'a'..='z' => *overlap as u32 - 96,  // a-z = 1-26
            _ => bail!("Invalid backpack item")
        };
        item_priorities.push(priority);
    }
    Ok(item_priorities.iter().sum())
}

fn identify_group_badge<I>(lines: I) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut group_priorities = vec![];
    for chunk in &lines.into_iter().chunks(3) {
        let mut group_backpacks = vec![];
        for line in chunk {
            let line = line.as_ref();
            let backpack: HashSet<char> = HashSet::from_iter(line.chars());
            group_backpacks.push(backpack);
        }
        let first_intersection = HashSet::from_iter(group_backpacks[0].intersection(&group_backpacks[1]).into_iter().map(|c| *c));
        let total_intersection = first_intersection.intersection(&group_backpacks[2]).take(1).nth(0).context("No matching group badge")?;
        let priority = match total_intersection {
            'A'..='Z' => *total_intersection as u32 - 38,  // A-Z = 27-52
            'a'..='z' => *total_intersection as u32 - 96,  // a-z = 1-26
            _ => bail!("Invalid backpack item")
        };
        group_priorities.push(priority);
    }
    Ok(group_priorities.iter().sum())
}

fn day1() -> Result<u32> {
    get_max(read_lines!("day1.txt"), 3).context("Getting calorie inventory")
}

fn day2() -> Result<u32> {
    calculate_rps_score(read_lines!("day2.txt")).context("Calculating RPS scores")
}

fn day3() -> Result<()> {
    let a = calculate_backpack_score(read_lines!("day3.txt")).context("Calculating backpack score")?;
    let b = identify_group_badge(read_lines!("day3.txt")).context("Calculating group badges")?;
    println!("Day 3: {}", a);
    println!("Day 3: {}", b);
    Ok(())
}

fn main() -> Result<()> {
    println!("Day 1: {}", day1()?);
    println!("Day 2: {}", day2()?);
    day3()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day1() -> Result<()> {
        assert_eq!(get_max(vec!["1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "", "10000"], 1)?, 24000);
        assert_eq!(get_max(vec!["1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "", "10000"], 3)?, 45000);
        assert_eq!(day1()?, 208567);
        Ok(())
    }
    #[test]
    fn test_day2() -> Result<()> {
        assert_eq!(calculate_rps_score(vec!["A Y", "B X", "C Z"])?, 12);
        assert_eq!(day2()?, 14859);
        Ok(())
    }
    #[test]
    fn test_day3() -> Result<()> {
        assert_eq!(calculate_backpack_score(vec!["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"])?, 157);
        assert_eq!(identify_group_badge(vec!["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"])?, 70);
        Ok(())
    }
}
