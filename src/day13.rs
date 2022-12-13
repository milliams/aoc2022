use std::cmp::Ordering;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{Context, Result};
use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::map_res,
    multi::separated_list0,
    sequence::delimited,
    Finish,
};

use crate::read_lines;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Message {
    Num(u32),
    List(Vec<Message>),
}

impl Message {
    fn num_from_str(s: &str) -> Result<Message> {
        Ok(Message::Num(u32::from_str(s)?))
    }
}

impl FromStr for Message {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_list(s).finish() {
            Ok((_remaining, message)) => Ok(message),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Message::Num(a), Message::Num(b)) => a.cmp(b),
            (Message::Num(a), Message::List(b)) => vec![Message::Num(*a)].cmp(b),
            (Message::List(a), Message::Num(b)) => a.cmp(&vec![Message::Num(*b)]),
            (Message::List(a), Message::List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_list_entries(s: &str) -> nom::IResult<&str, Message> {
    let (remaining, list) = separated_list0(
        delimited(multispace0, char(','), multispace0),
        alt((parse_number, parse_list)),
    )(s)?;
    Ok((remaining, Message::List(list)))
}

fn parse_list(s: &str) -> nom::IResult<&str, Message> {
    delimited(tag("["), parse_list_entries, tag("]"))(s)
}

fn parse_number(s: &str) -> nom::IResult<&str, Message> {
    map_res(digit1, Message::num_from_str)(s)
}

fn check_message<I>(lines: I) -> Result<usize>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut index_sum = 0;
    for block in &lines.into_iter().enumerate().chunks(3) {
        let lines = block
            .take(2)
            .map(|(i, l)| (i, l.as_ref().to_string()))
            .collect_vec();
        let index = (lines[0].0 / 3) + 1;
        let a: Message = lines[0].1.parse()?;
        let b: Message = lines[1].1.parse()?;

        if a < b {
            index_sum += index;
        }
    }
    Ok(index_sum)
}

fn sort_messages<I>(lines: I) -> Result<usize>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let new_messages: Vec<Message> = vec!["[[2]]".parse()?, "[[6]]".parse()?];
    let all_messages = lines
        .into_iter()
        .filter_map(|l| l.as_ref().to_string().parse().ok())
        .chain(new_messages.clone())
        .sorted()
        .enumerate()
        .filter_map(|(i, m)| {
            if new_messages.contains(&m) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();
    Ok(all_messages)
}

pub fn day13() -> Result<(usize, usize)> {
    let a = check_message(read_lines!("day13.txt")).context("checking messages")?;
    let b = sort_messages(read_lines!("day13.txt")).context("sorting messages")?;
    Ok((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day() -> Result<()> {
        assert_eq!(parse_number("1")?.1, Message::Num(1));
        assert_eq!(parse_number("12")?.1, Message::Num(12));
        assert_eq!(
            parse_list_entries("1,2,3")?.1,
            Message::List(vec![Message::Num(1), Message::Num(2), Message::Num(3)])
        );
        assert_eq!(
            parse_list_entries("1, 2, 3")?.1,
            Message::List(vec![Message::Num(1), Message::Num(2), Message::Num(3)])
        );
        assert_eq!(
            parse_list("[1,2,3]")?.1,
            Message::List(vec![Message::Num(1), Message::Num(2), Message::Num(3)])
        );
        assert_eq!(
            parse_list("[1,[2]]")?.1,
            Message::List(vec![Message::Num(1), Message::List(vec![Message::Num(2)])])
        );
        assert_eq!(
            "[1,[2]]".parse::<Message>()?,
            Message::List(vec![Message::Num(1), Message::List(vec![Message::Num(2)])])
        );

        assert!("[1,1,3,1,1]".parse::<Message>()? < "[1,1,5,1,1]".parse::<Message>()?);
        assert!("[[1],[2,3,4]]".parse::<Message>()? < "[[1],4]".parse::<Message>()?);
        assert!("[9]".parse::<Message>()? > "[[8,7,6]]".parse::<Message>()?);
        assert!("[[4,4],4,4]".parse::<Message>()? < "[[4,4],4,4,4]".parse::<Message>()?);
        assert!("[7,7,7,7]".parse::<Message>()? > "[7,7,7]".parse::<Message>()?);
        assert!("[]".parse::<Message>()? < "[3]".parse::<Message>()?);
        assert!("[[[]]]".parse::<Message>()? > "[[]]".parse::<Message>()?);
        assert!(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Message>()?
                > "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Message>()?
        );

        let data = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(check_message(data.lines())?, 13);
        assert_eq!(sort_messages(data.lines())?, 140);

        assert_eq!(day13()?, (5208, 25792));

        Ok(())
    }
}
