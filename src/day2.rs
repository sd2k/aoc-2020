use std::str::FromStr;

use anyhow::{Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use serde::Deserialize;
use serde_scan::scan;

#[derive(Deserialize)]
struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.letter).count();
        count >= self.min && count <= self.max
    }
    fn is_actually_valid(&self) -> bool {
        let first = self
            .password
            .chars()
            .nth(self.min - 1)
            .map_or(false, |x| x == self.letter);
        let second = self
            .password
            .chars()
            .nth(self.max - 1)
            .map_or(false, |x| x == self.letter);
        first ^ second
    }
}

impl FromStr for Password {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(scan!("{}-{} {}: {}" <- s)?)
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Password> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Password]) -> u32 {
    input
        .iter()
        .filter_map(|x| if Password::is_valid(x) { Some(1) } else { None })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Password]) -> u32 {
    input
        .iter()
        .filter_map(|x| {
            if Password::is_actually_valid(x) {
                Some(1)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            )),
            2
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            )),
            1
        );
    }
}
