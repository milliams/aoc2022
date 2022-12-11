use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use itertools::Itertools;

use crate::read_lines;

#[derive(Debug)]
enum Operation {
    Multiply(u64),
    Square,
    Add(u64),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    target: (usize, usize),
    items_inspected: u64,
}

fn monkey_business<I>(lines: I, relief: bool, rounds: u32) -> Result<u64>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut monkeys = vec![];
    for block in &lines.into_iter().chunks(7) {
        let lines = block.map(|l| l.as_ref().to_string()).collect_vec();
        let items: Result<Vec<u64>, std::num::ParseIntError> = lines[1]
            .replace("  Starting items: ", "")
            .replace(',', "")
            .split(' ')
            .map(|n| n.replace(' ', "").parse())
            .collect();
        let operation = match lines[2]
            .replace("  Operation: new = old ", "")
            .split(' ')
            .collect_vec()
            .as_slice()
        {
            ["*", "old"] => Operation::Square,
            ["*", x] => Operation::Multiply(x.parse()?),
            ["+", x] => Operation::Add(x.parse()?),
            _ => bail!("unrecognised operation {}", lines[2]),
        };
        let test = lines[3]
            .replace("  Test: divisible by ", "")
            .parse()
            .context("parsing divisibility test")?;
        let target_true = lines[4]
            .replace("    If true: throw to monkey ", "")
            .parse()?;
        let target_false = lines[5]
            .replace("    If false: throw to monkey ", "")
            .parse()?;
        let monkey = Monkey {
            items: items.context("getting starting items")?,
            operation,
            test,
            target: (target_true, target_false),
            items_inspected: 0,
        };
        monkeys.push(monkey);
    }

    let moderator: u64 = monkeys.iter().map(|m| m.test).product();

    for _ in 1..=rounds {
        for m in 0..monkeys.len() {
            let mut true_pass = vec![];
            let mut false_pass = vec![];
            let target;
            {
                let monkey = &mut monkeys[m];
                for item in &monkey.items {
                    let mut worry = *item;
                    match monkey.operation {
                        Operation::Multiply(x) => worry *= x,
                        Operation::Square => worry *= worry,
                        Operation::Add(x) => worry += x,
                    }

                    if relief {
                        worry /= 3;
                    }
                    worry %= moderator;

                    if worry % monkey.test == 0 {
                        true_pass.push(worry);
                    } else {
                        false_pass.push(worry);
                    }
                    monkey.items_inspected += 1;
                }
                target = monkey.target;
                monkey.items.clear();
            }

            monkeys[target.0].items.extend(true_pass);
            monkeys[target.1].items.extend(false_pass);
        }
    }

    let monkey_business = monkeys
        .iter()
        .map(|m| m.items_inspected)
        .sorted()
        .rev()
        .take(2)
        .product();
    Ok(monkey_business)
}

pub fn day11() -> Result<(u64, u64)> {
    let a = monkey_business(read_lines!("day11.txt"), true, 20).context("")?;
    let b = monkey_business(read_lines!("day11.txt"), false, 10000).context("")?;
    Ok((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use textwrap::dedent;
    #[test]
    fn test_day5() -> Result<()> {
        let test_data = "
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1";
        assert_eq!(
            monkey_business(dedent(test_data).lines().skip(1), true, 20)?,
            10605
        );
        assert_eq!(
            monkey_business(dedent(test_data).lines().skip(1), false, 10000)?,
            2713310158
        );
        Ok(())
    }
}
