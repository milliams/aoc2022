use std::collections::HashSet;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

use crate::read_lines;

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
        let overlap = left
            .intersection(&right)
            .take(1)
            .next()
            .context("No misplaced item")?;
        let priority = match overlap {
            'A'..='Z' => *overlap as u32 - 38, // A-Z = 27-52
            'a'..='z' => *overlap as u32 - 96, // a-z = 1-26
            _ => bail!("Invalid backpack item"),
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
        let first_intersection = HashSet::from_iter(
            group_backpacks[0]
                .intersection(&group_backpacks[1])
                .into_iter()
                .copied(),
        );
        let total_intersection = first_intersection
            .intersection(&group_backpacks[2])
            .take(1)
            .next()
            .context("No matching group badge")?;
        let priority = match total_intersection {
            'A'..='Z' => *total_intersection as u32 - 38, // A-Z = 27-52
            'a'..='z' => *total_intersection as u32 - 96, // a-z = 1-26
            _ => bail!("Invalid backpack item"),
        };
        group_priorities.push(priority);
    }
    Ok(group_priorities.iter().sum())
}

pub fn day3() -> Result<()> {
    let a =
        calculate_backpack_score(read_lines!("day3.txt")).context("Calculating backpack score")?;
    let b = identify_group_badge(read_lines!("day3.txt")).context("Calculating group badges")?;
    println!("Day 3: {}", a);
    println!("Day 3: {}", b);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day3() -> Result<()> {
        assert_eq!(
            calculate_backpack_score(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ])?,
            157
        );
        assert_eq!(
            identify_group_badge(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ])?,
            70
        );
        Ok(())
    }
}
