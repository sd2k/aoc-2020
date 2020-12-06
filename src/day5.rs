use std::str::FromStr;

use anyhow::{Context, Error, Result};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Row(u16);

impl FromStr for Row {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(u16::from_str_radix(
            &s.replace('F', "0").replace('B', "1"),
            2,
        )?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Column(u16);

impl FromStr for Column {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(u16::from_str_radix(
            &s.replace('L', "0").replace('R', "1"),
            2,
        )?))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Seat {
    row: Row,
    column: Column,
}

impl Seat {
    fn id(self) -> u32 {
        (self.row.0 * 8 + self.column.0).into()
    }
}

impl FromStr for Seat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            row: s
                .get(..7)
                .context("Seat spec too short!")
                .and_then(|x| Ok(x.parse()?))?,
            column: s
                .get(7..)
                .context("Seat spec too short!")
                .and_then(|x| Ok(x.parse()?))?,
        })
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Seat> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[Seat]) -> u32 {
    input.iter().map(|s| s.id()).max().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &[Seat]) -> u32 {
    // We can determine the missing number by calculating the sum of (min..max)
    // and comparing it to the observed sum.
    let (min, max, observed_sum, len) = input
        .iter()
        .map(|x| x.id())
        .fold((u32::MAX, u32::MIN, 0, 0), |(min, max, sum, len), x| {
            (min.min(x), max.max(x), sum + x, len + 1)
        });
    // Use Gauss's method for calculating the sum of an arithmetic series.
    ((len + 1) * (min + max) / 2) - observed_sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_row() {
        assert_eq!("FBFBBFF".parse::<Row>().unwrap(), Row(44u16))
    }

    #[test]
    fn parse_column() {
        assert_eq!("RLR".parse::<Column>().unwrap(), Column(5u16))
    }

    #[test]
    fn parse_seat() {
        assert_eq!(
            "FBFBBFFRLR".parse::<Seat>().unwrap(),
            Seat {
                row: Row(44u16),
                column: Column(5u16),
            }
        );
        assert_eq!(
            "BFFFBBFRRR".parse::<Seat>().unwrap(),
            Seat {
                row: Row(70u16),
                column: Column(7u16),
            }
        );
        assert_eq!(
            "FFFBBBFRRR".parse::<Seat>().unwrap(),
            Seat {
                row: Row(14u16),
                column: Column(7u16),
            }
        );
        assert_eq!(
            "BBFFBBFRLL".parse::<Seat>().unwrap(),
            Seat {
                row: Row(102u16),
                column: Column(4u16),
            }
        );
    }

    #[test]
    fn seat_id() {
        assert_eq!("FBFBBFFRLR".parse::<Seat>().unwrap().id(), 357,);
        assert_eq!("BFFFBBFRRR".parse::<Seat>().unwrap().id(), 567);
        assert_eq!("FFFBBBFRRR".parse::<Seat>().unwrap().id(), 119);
        assert_eq!("BBFFBBFRLL".parse::<Seat>().unwrap().id(), 820);
    }
}
