use std::collections::VecDeque;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use crate::read_lines;

fn reorder_stacks<I>(lines: I) -> Result<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut stacks = vec![];
    let mut defining_stacks = true;
    for line in lines {
        let line = line.as_ref();
        if defining_stacks {
            let num_stacks = ((line.len() + 1) / 4) as usize;
            if stacks.is_empty() {
                stacks.resize(num_stacks, VecDeque::new());
            }
            let line_vec: Vec<char> = line.chars().collect();
            for stack_num in 1..=num_stacks {
                let stack_position = (stack_num * 4) - 3;
                let crate_code = line_vec[stack_position];
                if crate_code == '1' {
                    break;
                }
                if crate_code != ' ' {
                    stacks[stack_num - 1].push_front(crate_code);
                }
            }
        } else {
            let moves: Vec<_> = line.split(' ').collect();
            let number = moves[1].parse()?;
            let from: usize = moves[3].parse()?;
            let to: usize = moves[5].parse()?;
            let mut staging: VecDeque<char> = VecDeque::new();
            for _ in 0..number {
                let moving_crate = stacks[from - 1]
                    .pop_back()
                    .context("Getting crate from stack")?;
                staging.push_back(moving_crate);
            }
            for _ in 0..number {
                let moving_crate = staging.pop_back().context("Getting crate from stack")?;
                stacks[to - 1].push_back(moving_crate);
            }
        }
        //stacks.push(line);
        if line.is_empty() {
            defining_stacks = false;
        }
    }
    stacks
        .iter()
        .map(|s| s.back().context("Getting top crate"))
        .collect()
}

pub fn day5() -> Result<String> {
    reorder_stacks(read_lines!("day05.txt")).context("Reordering crate stacks")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day5() -> Result<()> {
        assert_eq!(
            reorder_stacks(vec![
                "    [D]    ",
                "[N] [C]    ",
                "[Z] [M] [P]",
                " 1   2   3 ",
                "",
                "move 1 from 2 to 1",
                "move 3 from 1 to 3",
                "move 2 from 2 to 1",
                "move 1 from 1 to 2"
            ])?,
            "MCD"
        );
        assert_eq!(day5()?, "BRQWDBBJM");
        Ok(())
    }
}
