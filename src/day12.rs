use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use ndarray::{stack, Array1, Array2, Axis};
use pathfinding::directed::bfs::bfs;

use crate::read_lines;

#[derive(Debug)]
enum Connection {
    Up,
    Down,
    Left,
    Right,
}

type Heightmap = Array2<u8>;
type Graph = Array2<[Option<Connection>; 4]>;

fn lines_to_grid<I>(lines: I) -> Result<(Heightmap, Graph, (usize, usize), (usize, usize))>
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
                        _ => bail!(""),
                    }
                })
                .collect()
        })
        .collect();
    let heightmap = heightmap?;
    let heightmap: Vec<_> = heightmap.iter().map(|x| x.view()).collect();
    let heightmap = stack(Axis(0), &heightmap)?;

    let graph_vec: Vec<[Option<Connection>; 4]> = heightmap
        .indexed_iter()
        .map(|((row, col), h)| {
            let up = if row > 0 && heightmap[(row - 1, col)] <= h + 1 {
                Some(Connection::Up)
            } else {
                None
            };
            let left = if col > 0 && heightmap[(row, col - 1)] <= h + 1 {
                Some(Connection::Left)
            } else {
                None
            };
            let down = if row < heightmap.shape()[0] - 1 && heightmap[(row + 1, col)] <= h + 1 {
                Some(Connection::Down)
            } else {
                None
            };
            let right = if col < heightmap.shape()[1] - 1 && heightmap[(row, col + 1)] <= h + 1 {
                Some(Connection::Right)
            } else {
                None
            };
            [up, down, left, right]
        })
        .collect();
    let graph = Array2::from_shape_vec([heightmap.shape()[0], heightmap.shape()[1]], graph_vec)
        .context("")?;
    Ok((
        heightmap,
        graph,
        start.context("getting start")?,
        end.context("getting end")?,
    ))
}

fn find_path(
    graph: &Graph,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    bfs(
        &start,
        |p| {
            graph[*p]
                .iter()
                .filter_map(|c| match c {
                    Some(Connection::Up) => Some((p.0 - 1, p.1)),
                    Some(Connection::Down) => Some((p.0 + 1, p.1)),
                    Some(Connection::Left) => Some((p.0, p.1 - 1)),
                    Some(Connection::Right) => Some((p.0, p.1 + 1)),
                    None => None,
                })
                .collect_vec()
        },
        |p| *p == end,
    )
}

fn find_shortest_from_height(
    graph: &Graph,
    heightmap: &Heightmap,
    height: u32,
    end: (usize, usize),
) -> Result<usize> {
    graph
        .indexed_iter()
        .filter_map(|(c, _)| {
            let h = heightmap[c] as u32;
            if h == height {
                find_path(graph, c, end)
            } else {
                None
            }
        })
        .map(|path| path.len() - 1)
        .min()
        .context("finding shortest start")
}

fn solve_maze<I>(lines: I) -> Result<(usize, usize)>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let (heightmap, graph, start, end) = lines_to_grid(lines)?;
    let start_to_end = find_path(&graph, start, end);

    let shortest_from_start_level = find_shortest_from_height(&graph, &heightmap, 0, end)?;

    Ok((
        start_to_end.context("")?.len() - 1,
        shortest_from_start_level,
    ))
}

pub fn day12() -> Result<(usize, usize)> {
    solve_maze(read_lines!("day12.txt")).context("solving maze")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day() -> Result<()> {
        let test_data = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let (heightmap, graph, start, end) = lines_to_grid(test_data.lines())?;
        assert_eq!(start, (0, 0));
        assert_eq!(end, (2, 5));
        assert_eq!(find_path(&graph, start, end).context("")?.len() - 1, 31);

        assert_eq!(find_shortest_from_height(&graph, &heightmap, 0, end)?, 29);

        Ok(())
    }
}
