use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Context, Result};
use ndarray::{stack, Array1, Array2, Axis};
use pathfinding::directed::bfs::bfs;

use crate::read_lines;

type Heightmap = Array2<u8>;
type Pos = (usize, usize);

fn lines_to_grid<I>(lines: I) -> Result<(Heightmap, Pos, Pos)>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut start = None;
    let mut end = None;
    let heightmap: Result<Vec<Array1<u8>>> = lines
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.as_ref()
                .chars()
                .enumerate()
                .map(|(col, d)| {
                    let d = d as u8;
                    match d {
                        b'a'..=b'z' => Ok(d - b'a'),
                        b'S' => {
                            start = Some((row, col));
                            Ok(0)
                        }
                        b'E' => {
                            end = Some((row, col));
                            Ok(b'z' - b'a')
                        }
                        _ => Err(anyhow!("symbol not found")),
                    }
                })
                .collect()
        })
        .collect();
    let heightmap = heightmap?;
    let heightmap: Vec<_> = heightmap.iter().map(|x| x.view()).collect();
    let heightmap = stack(Axis(0), &heightmap)?;

    Ok((
        heightmap,
        start.context("getting start")?,
        end.context("getting end")?,
    ))
}

fn find_path(graph: &Heightmap, start: Pos, end: Pos) -> Option<Vec<Pos>> {
    bfs(
        &start,
        |p| {
            let (row, col) = *p;
            let h = graph[*p];
            let mut successors = vec![];
            if row > 0 && graph[(row - 1, col)] <= h + 1 {
                successors.push((row - 1, col))
            }
            if col > 0 && graph[(row, col - 1)] <= h + 1 {
                successors.push((row, col - 1))
            }
            if row < graph.shape()[0] - 1 && graph[(row + 1, col)] <= h + 1 {
                successors.push((row + 1, col))
            }
            if col < graph.shape()[1] - 1 && graph[(row, col + 1)] <= h + 1 {
                successors.push((row, col + 1))
            }
            successors
        },
        |p| *p == end,
    )
}

fn find_shortest_from_height(heightmap: &Heightmap, height: u32, end: Pos) -> Result<usize> {
    heightmap
        .indexed_iter()
        .filter_map(|(c, h)| {
            if *h as u32 == height {
                find_path(heightmap, c, end)
            } else {
                None
            }
        })
        .map(|path| path.len() - 1)
        .min()
        .context("finding shortest start")
}

fn solve_maze<I>(lines: I) -> Result<Pos>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let (heightmap, start, end) = lines_to_grid(lines)?;
    let start_to_end = find_path(&heightmap, start, end);

    let shortest_from_start_level = find_shortest_from_height(&heightmap, 0, end)?;

    Ok((
        start_to_end.context("finding primary path")?.len() - 1,
        shortest_from_start_level,
    ))
}

pub fn day12() -> Result<Pos> {
    solve_maze(read_lines!("day12.txt")).context("solving maze")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day() -> Result<()> {
        let test_data = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];
        let (heightmap, start, end) = lines_to_grid(test_data)?;
        assert_eq!(start, (0, 0));
        assert_eq!(end, (2, 5));
        assert_eq!(find_path(&heightmap, start, end).context("running test")?.len() - 1, 31);
        assert_eq!(find_shortest_from_height(&heightmap, 0, end)?, 29);

        assert_eq!(day12()?, (412, 402));

        Ok(())
    }
}
