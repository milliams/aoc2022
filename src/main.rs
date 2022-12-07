use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

macro_rules! read_lines {
    ($expression:expr) => {
        BufReader::new(std::fs::File::open($expression)?)
            .lines()
            .filter(|l| l.is_ok())
            .map(|l| l.unwrap())
    };
}

pub(crate) use read_lines;

fn main() -> Result<()> {
    println!("Day 1: {}", day1::day1()?);
    println!("Day 2: {}", day2::day2()?);
    day3::day3()?;
    println!("Day 4: {}", day4::day4()?);
    println!("Day 5: {}", day5::day5()?);
    println!("Day 6: {}", day6::day6()?);
    println!("Day 7: {:?}", day7::day7()?);

    Ok(())
}
