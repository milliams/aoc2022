use std::collections::HashSet;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};

use crate::read_lines;

#[derive(PartialEq, Debug)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

fn parse_move(text: &str) -> Result<(Dir, u32)> {
    let movement_parts: Vec<_> = text.split(' ').collect();
    let m = match movement_parts.as_slice() {
        [dir, distance] => {
            let dir = match *dir {
                "U" => Dir::Up,
                "D" => Dir::Down,
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => bail!(""),
            };
            (dir, distance.parse()?)
        }
        _ => bail!(""),
    };
    Ok(m)
}

fn tail_coverage<I>(lines: I, length: usize) -> Result<usize>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let start = (0, 0);
    let mut rope = vec![start; length];
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert(start);
    for line in lines {
        let line = line.as_ref();
        let (dir, num_steps) = parse_move(line)?;

        for _ in 0..num_steps {
            match dir {
                Dir::Up => rope[0].1 += 1,
                Dir::Down => rope[0].1 -= 1,
                Dir::Right => rope[0].0 += 1,
                Dir::Left => rope[0].0 -= 1,
            }

            for i in 1..rope.len() {
                let lead = rope[i - 1];
                let mut tail = rope[i];
                let gap = (lead.0 - tail.0, lead.1 - tail.1);
                tail = match gap {
                    (2, 2) => (lead.0 - 1, lead.1 - 1),
                    (-2, -2) => (lead.0 - -1, lead.1 - -1),
                    (2, -2) => (lead.0 - 1, lead.1 - -1),
                    (-2, 2) => (lead.0 - -1, lead.1 - 1),
                    (2, x) if (-1..=1).contains(&x) => (lead.0 - 1, lead.1),
                    (-2, x) if (-1..=1).contains(&x) => (lead.0 - -1, lead.1),
                    (x, 2) if (-1..=1).contains(&x) => (lead.0, lead.1 - 1),
                    (x, -2) if (-1..=1).contains(&x) => (lead.0, lead.1 - -1),
                    (1, x) if (-1..=1).contains(&x) => tail,
                    (-1, x) if (-1..=1).contains(&x) => tail,
                    (x, 1) if (-1..=1).contains(&x) => tail,
                    (x, -1) if (-1..=1).contains(&x) => tail,
                    (0, 0) => tail,
                    _ => bail!("planck length exceeded"),
                };
                rope[i] = tail;
            }
            tail_visited.insert(*rope.last().context("getting tail")?);
        }
    }
    Ok(tail_visited.len())
}

pub fn day9() -> Result<(usize, usize)> {
    let a = tail_coverage(read_lines!("day09.txt"), 2).context("getting tail coverage")?;
    let b = tail_coverage(read_lines!("day09.txt"), 10).context("getting tail coverage")?;
    Ok((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day5() -> Result<()> {
        assert_eq!(parse_move("R 4")?, (Dir::Right, 4));
        assert_eq!(parse_move("U 56374")?, (Dir::Up, 56374));
        assert_eq!(
            tail_coverage(
                vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"],
                2
            )?,
            13
        );
        assert_eq!(
            tail_coverage(
                vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"],
                10
            )?,
            1
        );

        assert_eq!(
            tail_coverage(
                vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"],
                10
            )?,
            36
        );
        assert_eq!(day9()?, (6311, 2482));
        Ok(())
    }
}
