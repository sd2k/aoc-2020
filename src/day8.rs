use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

use aoc_runner_derive::{aoc, aoc_generator};

fn parse_int_with_leading_plus(s: &str) -> Result<i32> {
    Ok(s.trim_start_matches('+').parse()?)
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    fn swap_nop_jmp(&mut self) {
        match self {
            Instruction::Jmp(x) => *self = Instruction::Nop(*x),
            Instruction::Nop(x) => *self = Instruction::Jmp(*x),
            _ => unreachable!(),
        };
    }
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split(' ').rev();
        let arg = iter
            .next()
            .context("No argument found!")
            .and_then(parse_int_with_leading_plus)?;
        match iter.next() {
            Some("nop") => Ok(Self::Nop(arg)),
            Some("acc") => Ok(Self::Acc(arg)),
            Some("jmp") => Ok(Self::Jmp(arg)),
            Some(instruction) => bail!("Invalid instruction found: {}", instruction),
            None => bail!("No instruction found!"),
        }
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

#[derive(Debug)]
struct State {
    acc: i32,
    hits: Vec<bool>,
    instruction: i32,
}

impl State {
    fn new(len: usize) -> Self {
        Self {
            acc: 0,
            hits: vec![false; len],
            instruction: 0,
        }
    }

    fn step(&mut self, instruction: Instruction) -> Option<i32> {
        let idx = self.instruction as usize;
        if self.hits[idx] {
            return Some(self.acc);
        }
        self.hits[idx] = true;
        match instruction {
            Instruction::Nop(_) => self.instruction += 1,
            Instruction::Acc(x) => {
                self.acc += x;
                self.instruction += 1;
            }
            Instruction::Jmp(x) => self.instruction += x,
        }
        None
    }
}

#[aoc(day8, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let mut state = State::new(input.len());
    loop {
        let instruction = input[state.instruction as usize];
        if let Some(acc) = state.step(instruction) {
            return acc;
        }
    }
}

#[aoc(day8, part2)]
fn part2(input: &[Instruction]) -> i32 {
    let len = input.len();
    let mut input = input.to_vec();
    let mut changed = 0;
    loop {
        // Get the index of the instruction to change.
        // This lets us change the instruction back later to avoid cloning
        // the entire set of instructions every iteration. (Otherwise we'd
        // need a mutable reference which wouldn't let us access the input
        // immutably later).
        let to_change_idx = input
            .iter()
            .enumerate()
            .filter(|(_i, x)| matches!(x, Instruction::Nop(_) | Instruction::Jmp(_)))
            .nth(changed)
            .unwrap()
            .0;
        input[to_change_idx].swap_nop_jmp();
        changed += 1;

        let mut state = State::new(len);
        loop {
            if state.instruction as usize == len {
                return state.acc;
            }
            let instruction = input[state.instruction as usize];
            if state.step(instruction).is_some() {
                input[to_change_idx].swap_nop_jmp();
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            )),
            5
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            )),
            8
        );
    }
}
