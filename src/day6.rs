use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use itertools::Itertools;

use crate::read_lines;

enum MessageMarker {
    StartOfPacket,
    StartOfMessage,
}

fn find_start_marker<S: ToString>(message: S, marker_type: MessageMarker) -> Result<usize> {
    let mut answer = None;
    let window_size = match marker_type {
        MessageMarker::StartOfPacket => 4,
        MessageMarker::StartOfMessage => 14,
    };
    for (i, window) in message
        .to_string()
        .as_bytes()
        .windows(window_size)
        .enumerate()
    {
        let unique = &window.iter().unique().collect::<Vec<&u8>>().len();
        if unique == &window.len() {
            answer = Some(i + window_size);
            break;
        }
    }
    answer.context("getting marker")
}

pub fn day6() -> Result<usize> {
    let a = find_start_marker(
        read_lines!("day6.txt")
            .next()
            .context("getting line from file")?,
        MessageMarker::StartOfMessage,
    )
    .context("finding start-of-packet marker");
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day4() -> Result<()> {
        assert_eq!(
            find_start_marker(
                "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                MessageMarker::StartOfPacket
            )?,
            7
        );
        assert_eq!(
            find_start_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", MessageMarker::StartOfPacket)?,
            5
        );
        assert_eq!(
            find_start_marker("nppdvjthqldpwncqszvftbrmjlhg", MessageMarker::StartOfPacket)?,
            6
        );
        assert_eq!(
            find_start_marker(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                MessageMarker::StartOfPacket
            )?,
            10
        );
        assert_eq!(
            find_start_marker(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                MessageMarker::StartOfPacket
            )?,
            11
        );
        assert_eq!(
            find_start_marker(
                read_lines!("day6.txt")
                    .next()
                    .context("getting line from file")?,
                MessageMarker::StartOfPacket
            )?,
            1361
        );

        assert_eq!(
            find_start_marker(
                "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                MessageMarker::StartOfMessage
            )?,
            19
        );
        assert_eq!(
            find_start_marker(
                "bvwbjplbgvbhsrlpgdmjqwftvncz",
                MessageMarker::StartOfMessage
            )?,
            23
        );
        assert_eq!(
            find_start_marker(
                "nppdvjthqldpwncqszvftbrmjlhg",
                MessageMarker::StartOfMessage
            )?,
            23
        );
        assert_eq!(
            find_start_marker(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                MessageMarker::StartOfMessage
            )?,
            29
        );
        assert_eq!(
            find_start_marker(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                MessageMarker::StartOfMessage
            )?,
            26
        );
        assert_eq!(
            find_start_marker(
                read_lines!("day6.txt")
                    .next()
                    .context("getting line from file")?,
                MessageMarker::StartOfMessage
            )?,
            3263
        );

        Ok(())
    }
}
