use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use crate::read_lines;

fn overlapping_assignments<I>(lines: I, include_partial: bool) -> Result<u32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut num_overlap = 0;
    for line in lines {
        let line = line.as_ref();
        let mut splits = line.split(",").take(2);
        let a = splits.nth(0).context("no Elf 1")?;
        let b = splits.nth(0).context("no Elf 2")?;
        let mut a = a.split("-").take(2);
        let mut b = b.split("-").take(2);
        let a_start: u32 = a.nth(0).context("no Elf 1 start")?.parse()?;
        let a_end: u32 = a.nth(0).context("no Elf 1 end")?.parse()?;
        let b_start: u32 = b.nth(0).context("no Elf 2 start")?.parse()?;
        let b_end: u32 = b.nth(0).context("no Elf 2 end")?.parse()?;
        if include_partial {
            if (a_start <= b_start && a_end >= b_start) || (a_start <= b_end && a_end >= b_start) {
                num_overlap += 1;
            }
        } else {
            if (a_start <= b_start && a_end >= b_end) || (b_start <= a_start && b_end >= a_end) {
                num_overlap += 1;
            }
        }
    }

    Ok(num_overlap)
}

pub fn day4() -> Result<u32> {
    let a = overlapping_assignments(read_lines!("day4.txt"), true)
        .context("Calculating overlapping assignments");
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day4() -> Result<()> {
        assert_eq!(
            overlapping_assignments(
                vec!["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"],
                false
            )?,
            2
        );
        assert_eq!(
            overlapping_assignments(
                vec!["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"],
                true
            )?,
            4
        );
        assert_eq!(day4()?, 911);
        Ok(())
    }
}
