use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};

use crate::read_lines;

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
            _ => bail!("Invalid move"),
        };
        let desired_result = match line.chars().nth(2).context("No second char")? {
            'X' => GameResult::Lose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => bail!("Invalid strategy"),
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

pub fn day2() -> Result<u32> {
    calculate_rps_score(read_lines!("day2.txt")).context("Calculating RPS scores")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day2() -> Result<()> {
        assert_eq!(calculate_rps_score(vec!["A Y", "B X", "C Z"])?, 12);
        assert_eq!(day2()?, 14859);
        Ok(())
    }
}
