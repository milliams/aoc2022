use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

macro_rules! read_lines {
    ($expression:expr) => {
        BufReader::new(File::open($expression)?).lines().filter(|l| l.is_ok()).map(|l| l.unwrap())
    };
}

pub(crate) use read_lines;

fn main() -> Result<()> {
    println!("Day 1: {}", day1::day1()?);
    println!("Day 2: {}", day2::day2()?);
    day3::day3()?;
    println!("Day 4: {}", day4::day4()?);
    println!("Day 5: {}", day5::day5()?);

    Ok(())
}
