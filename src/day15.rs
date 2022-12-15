use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{Context, Error, Result, bail};
use regex::Regex;

use crate::read_lines;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Reading {
    sensor: (i64, i64),
    beacon: (i64, i64),
}

impl Reading {
    fn distance(&self) -> i64 {
        (self.sensor.0 - self.beacon.0).abs() + (self.sensor.1 - self.beacon.1).abs()
    }

    fn row_coverage(&self, row: i64) -> std::ops::RangeInclusive<i64> {
        let d = self.distance();
        let dy = (row - self.sensor.1).abs();
        let dx = d - dy;
        (self.sensor.0 - dx)..=(self.sensor.0 + dx)
    }
}

impl FromStr for Reading {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )?;
        let cap = re.captures(s).context("Getting captures")?;
        Ok(Reading {
            sensor: (cap[1].parse()?, cap[2].parse()?),
            beacon: (cap[3].parse()?, cap[4].parse()?),
        })
    }
}

fn row_coverage<I>(lines: I, check: i64, range: std::ops::RangeInclusive<i64>) -> Result<usize>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let readings: Result<Vec<Reading>> = lines.into_iter().map(|l| l.as_ref().parse()).collect();
    let readings = readings?;

    let beacons_in_row: HashSet<_> = readings.iter().filter_map(|r| {
        if r.beacon.1 == check {
            Some(r.beacon.0)
        } else {
            None
        }
    }).collect();

    let covered: Vec<_> = readings.iter().map(|r| {
        r.row_coverage(check)
    }).collect();

    let mut count = 0;
    for x in range {
        if covered.iter().map(|r| r.contains(&x)).any(|c| c) && !beacons_in_row.contains(&x) {
            count += 1;
        }
    }

    Ok(count)
}

fn find_gap<I>(lines: I, range: std::ops::RangeInclusive<i64>) -> Result<i64>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let readings: Result<Vec<Reading>> = lines.into_iter().map(|l| l.as_ref().parse()).collect();
    let readings = readings?;

    for row in range.clone() {
        let covered: Vec<_> = readings.iter().map(|r| {
            r.row_coverage(row)
        }).collect();

        let mut x: i64 = *range.start();
        while x <= *range.end() {
            if let Some(in_range) = covered.iter().find(|r| r.contains(&x)) {
                x = *in_range.end();
            } else {
                return Ok(x*4000000+row)
            }
            x += 1;
        }
    }

    bail!("Could not find gap")
}

pub fn day15() -> Result<(usize, i64)> {
    let a = row_coverage(read_lines!("day15.txt"), 2000000, -5000000..=5000000).context("checking row coverage")?;
    let b = find_gap(read_lines!("day15.txt"), 0..=4000000).context("finding gap")?;
    Ok((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day() -> Result<()> {
        assert_eq!(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".parse::<Reading>()?,
            Reading {
                sensor: (2, 18),
                beacon: (-2, 15)
            }
        );
        let data = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(row_coverage(data.lines(), 10, -200..=200)?, 26);
        assert_eq!(find_gap(data.lines(), 0..=20)?, 56000011);

        assert_eq!(day15()?, (5181556, 12817603219131));

        Ok(())
    }
}
