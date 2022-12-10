use anyhow::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

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
    println!("Day  1: {}", day01::day1()?);
    println!("Day  2: {}", day02::day2()?);
    println!("Day  3: {:?}", day03::day3()?);
    println!("Day  4: {}", day04::day4()?);
    println!("Day  5: {}", day05::day5()?);
    println!("Day  6: {:?}", day06::day6()?);
    println!("Day  7: {:?}", day07::day7()?);
    println!("Day  8: {:?}", day08::day8()?);
    println!("Day  9: {:?}", day09::day9()?);
    let (d10a, d10b) = day10::day10()?;
    println!("Day 10: {}", d10a);
    println!("{}", d10b);

    Ok(())
}
