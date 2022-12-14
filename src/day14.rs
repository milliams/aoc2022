use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use itertools::Itertools;
use ndarray::{s, Array};

use crate::read_lines;

#[derive(Clone, Debug, PartialEq)]
enum C {
    Empty,
    Wall,
    Sand,
}

fn run_sand<I>(lines: I) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut lowest_point = 0;
    let mut grid = Array::from_elem((500, 1000), C::Empty);
    for line in lines {
        let line = line.as_ref();
        let path: Result<Vec<(usize, usize)>> = line
            .split(" -> ")
            .map(|s| {
                let parts = s.split(',').collect_vec();
                Ok((parts[0].parse()?, parts[1].parse()?))
            })
            .collect();
        let path = path?;
        for (start, stop) in path.iter().tuple_windows() {
            let x = if start.0 < stop.0 {
                start.0..=stop.0
            } else {
                stop.0..=start.0
            };
            let y = if start.1 < stop.1 {
                start.1..=stop.1
            } else {
                stop.1..=start.1
            };
            grid.slice_mut(s![y, x]).fill(C::Wall);
            lowest_point = lowest_point.max(start.1).max(stop.1);
        }
    }

    grid.slice_mut(s![lowest_point + 2, ..]).fill(C::Wall);

    let mut grain_num = 0;
    loop {
        let mut s = (0, 500);
        if grid[s] == C::Sand {
            break;
        }
        loop {
            let below = (s.0 + 1, s.1);
            let below_left = (s.0 + 1, s.1 - 1);
            let below_right = (s.0 + 1, s.1 + 1);
            if below.0 >= grid.shape()[0] {
                break;
            } else if grid[below] == C::Empty {
                s = below;
                continue;
            } else if grid[below_left] == C::Empty {
                s = below_left;
                continue;
            } else if grid[below_right] == C::Empty {
                s = below_right;
                continue;
            } else {
                grid[s] = C::Sand;
                break;
            }
        }
        grain_num += 1;
    }

    Ok(grain_num)
}

pub fn day14() -> Result<u32> {
    run_sand(read_lines!("day14.txt")).context("")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day() -> Result<()> {
        assert_eq!(
            run_sand([
                "498,4 -> 498,6 -> 496,6",
                "503,4 -> 502,4 -> 502,9 -> 494,9"
            ])?,
            93
        );
        assert_eq!(day14()?, 26461);
        Ok(())
    }
}
