use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use ndarray::{s, stack, Array1, Array2, Axis};

use crate::read_lines;

fn lines_to_grid<I>(lines: I) -> Result<Array2<u32>>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let grid: Result<Vec<Array1<u32>>> = lines
        .into_iter()
        .map(|line| {
            line.as_ref()
                .chars()
                .map(|d| d.to_digit(10).context(""))
                .collect()
        })
        .collect();
    let grid = grid?;
    let grid: Vec<_> = grid.iter().map(|x| x.view()).collect();
    let grid = stack(Axis(0), &grid)?;
    Ok(grid)
}

fn count_visible_trees<I>(lines: I) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let grid = lines_to_grid(lines)?;
    let num_visible = grid
        .indexed_iter()
        .map(|((x, y), h)| {
            let visible_left = grid.slice(s![x, ..y]).iter().all(|a| a < h);
            let visible_right = grid.slice(s![x, (y + 1)..]).iter().all(|a| a < h);
            let visible_above = grid.slice(s![..x, y]).iter().all(|a| a < h);
            let visible_below = grid.slice(s![(x + 1).., y]).iter().all(|a| a < h);
            visible_left || visible_right || visible_above || visible_below
        })
        .filter(|t| *t)
        .count();
    Ok(num_visible as u32)
}

fn max_scenic_score<I>(lines: I) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let grid = lines_to_grid(lines)?;
    let num_visible: usize = grid
        .indexed_iter()
        .map(|((x, y), h)| {
            [
                s![x, ..y;-1],
                s![x, (y + 1)..],
                s![..x;-1, y],
                s![(x + 1).., y],
            ]
            .iter()
            .map(|s| {
                let trees_left = grid.slice(s);
                if trees_left.iter().all(|a| a < h) {
                    trees_left.len()
                } else {
                    trees_left.iter().take_while(|t| h > t).count() + 1
                }
            })
            .product()
        })
        .max()
        .context("Finding max scenic score")?;
    Ok(num_visible as u32)
}

pub fn day8() -> Result<(u32, u32)> {
    let a = count_visible_trees(read_lines!("day08.txt")).context("")?;
    let b = max_scenic_score(read_lines!("day08.txt")).context("")?;
    Ok((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day5() -> Result<()> {
        assert_eq!(
            count_visible_trees(vec!["30373", "25512", "65332", "33549", "35390"])?,
            21
        );
        assert_eq!(
            max_scenic_score(vec!["30373", "25512", "65332", "33549", "35390"])?,
            8
        );
        assert_eq!(day8()?, (1789, 314820));
        Ok(())
    }
}
