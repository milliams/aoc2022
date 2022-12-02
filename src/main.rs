use std::fs::File;
use std::io::{BufReader, BufRead};

use anyhow::{Context, Result, bail};

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
        let should_win = match line.chars().nth(2).context("No second char")? {
            'X' => GameResult::Lose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => bail!("Invalid strategy")
        };

        let play = match (&other_move, &should_win) {
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

        let play_score = match should_win {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        };

        round_scores.push(shape_score + play_score);
    }
    Ok(round_scores.iter().sum())
}

fn day1() -> Result<u32> {
    get_max(read_lines!("day1.txt"), 3)
}

fn day2() -> Result<u32> {
    calculate_rps_score(read_lines!("day2.txt"))
}

fn main() -> Result<()> {
    println!("Day 1: {}", day1()?);
    println!("Day 2: {}", day2()?);

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
        assert_eq!(calculate_rps_score(vec!["G Y", "B X", "C Z"])?, 12);
        assert_eq!(day2()?, 14859);
        Ok(())
    }
}
