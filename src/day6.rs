use std::str::FromStr;

use anyhow::{Error, Result};

use aoc_runner_derive::{aoc, aoc_generator};

struct Group {
    n: u8,
    counts: [u8; 26]
}

impl Default for Group {
    fn default() -> Self {
        Group {
            n: 0,
            counts: [0; 26]
        }
    }
}

impl Group {
    fn any(&self) -> u32 {
        self.counts.iter().map(|x| if *x > 0 { 1 } else { 0 }).sum()
    }

    fn all(&self) -> u32 {
        self.counts.iter().map(|x| if *x == self.n { 1 } else { 0 }).sum()
    }
}

impl FromStr for Group {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(s.lines().fold(Group::default(), |mut group, line| {
            group.n += 1;
            line.bytes()
                .map(|c| c as usize - 97)
                .for_each(|idx| group.counts[idx] += 1);
            group
        }))
    }
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Group> {
    input.split("\n\n").map(|el| el.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Group]) -> u32 {
    input.iter().map(Group::any).sum()
}

#[aoc(day6, part2)]
fn part2(input: &[Group]) -> u32 {
    input.iter().map(Group::all).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            )),
            11
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            )),
            6
        );
    }
}
