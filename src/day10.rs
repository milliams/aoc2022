use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

use crate::read_lines;

enum Instruction {
    Noop,
    AddX(i32),
}

fn parse_instruction(text: &str) -> Result<Instruction> {
    let m = match text.split(' ').collect::<Vec<_>>().as_slice() {
        ["noop"] => Instruction::Noop,
        ["addx", v] => Instruction::AddX(v.parse()?),
        _ => bail!("invalid instruction '{}'", &text),
    };
    Ok(m)
}

fn run_computer<I>(lines: I) -> Result<(i32, String)>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut screen = [b'.'; 40 * 6];
    let mut signal_strengths = vec![];
    for line in lines {
        let line = line.as_ref();
        let i = parse_instruction(line)?;

        let num_cycles = match i {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        };

        for _ in 0..num_cycles {
            cycle += 1; // cycle starts

            // during cycle
            if (cycle - 20) % 40 == 0 {
                signal_strengths.push(cycle * x);
            }

            let pixel = cycle - 1;
            let h_pos = pixel % 40;
            if (h_pos - x).abs() <= 1 {
                screen[pixel as usize] = b'#';
            }
        }

        match i {
            Instruction::Noop => {}
            Instruction::AddX(v) => x += v,
        };
    }
    let screen = screen
        .chunks(40)
        .map(|c| std::str::from_utf8(c).unwrap_or("?"))
        .join("\n");
    Ok((signal_strengths.iter().sum(), screen))
}

pub fn day10() -> Result<(i32, String)> {
    run_computer(read_lines!("day10.txt")).context("getting tail coverage")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day5() -> Result<()> {
        let test_data = [
            "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
            "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5",
            "addx -1", "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1",
            "addx 16", "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop",
            "addx -3", "addx 9", "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop",
            "noop", "noop", "noop", "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop",
            "addx 2", "addx 6", "noop", "noop", "noop", "noop", "noop", "addx 1", "noop", "noop",
            "addx 7", "addx 1", "noop", "addx -13", "addx 13", "addx 7", "noop", "addx 1",
            "addx -33", "noop", "noop", "noop", "addx 2", "noop", "noop", "noop", "addx 8", "noop",
            "addx -1", "addx 2", "addx 1", "noop", "addx 17", "addx -9", "addx 1", "addx 1",
            "addx -3", "addx 11", "noop", "noop", "addx 1", "noop", "addx 1", "noop", "noop",
            "addx -13", "addx -19", "addx 1", "addx 3", "addx 26", "addx -30", "addx 12",
            "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9", "addx 18", "addx 1",
            "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1", "addx 2",
            "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22", "addx -6",
            "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop", "addx 20",
            "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
        ];
        let expected_signal_strength = 13140;
        let expected_screen = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....";
        assert_eq!(
            run_computer(test_data)?,
            (expected_signal_strength, expected_screen.to_string())
        );
        Ok(())
    }
}
