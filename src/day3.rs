use std::{convert::Infallible, ops::Deref, str::FromStr};

// use anyhow::{Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Coordinate {
    Open,
    Tree,
}

impl From<char> for Coordinate {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Tree,
            '.' => Self::Open,
            _ => panic!("Unexpected coordinate! {}", c),
        }
    }
}

#[derive(Debug)]
struct Row(Vec<Coordinate>);

impl Deref for Row {
    type Target = Vec<Coordinate>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Row {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(Coordinate::from).collect()))
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Row> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

fn n_trees(rows: &[Row], trajectory: &(usize, usize)) -> usize {
    let mut x = 0;
    let mut n_trees = 0;
    let width = rows[0].len();
    let iter = rows.iter().step_by(trajectory.1);
    for row in iter {
        let coordinate = row[x % width];
        if coordinate == Coordinate::Tree {
            n_trees += 1;
        }
        x += trajectory.0;
    }
    n_trees
}

#[aoc(day3, part1)]
fn part1(input: &[Row]) -> usize {
    n_trees(&input, &(3, 1))
}

#[aoc(day3, part2)]
fn part2(input: &[Row]) -> usize {
    let trajectories = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    trajectories
        .iter()
        .map(|traj| n_trees(&input, traj))
        .product()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            )),
            7
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            )),
            336
        );
    }
}
