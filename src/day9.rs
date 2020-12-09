use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

fn last_is_not_sum(window: impl Iterator<Item = u64> + Clone, sum: u64) -> bool {
    !window.tuple_combinations().any(|(x, y)| x + y == sum)
}

fn first_rulebreaker(input: &[u64], preamble: usize) -> u64 {
    input
        .windows(preamble + 1)
        .find_map(|w| {
            let last = *w.last().unwrap();
            if last_is_not_sum(w.iter().take(preamble).cloned(), last) {
                Some(last)
            } else {
                None
            }
        })
        .unwrap()
}

#[aoc(day9, part1)]
fn part1(input: &[u64]) -> u64 {
    first_rulebreaker(input, 25)
}

fn sum_contiguous_numbers(input: &[u64], sum: u64) -> u64 {
    for size in 2..(input.len() - 1) {
        for window in input.windows(size) {
            if window.iter().sum::<u64>() == sum {
                return window.iter().min().unwrap() + window.iter().max().unwrap();
            }
        }
    }
    panic!("Could not find contiguous numbers summing to {}", sum)
}

#[aoc(day9, part2)]
fn part2(input: &[u64]) -> u64 {
    let sum = first_rulebreaker(input, 25);
    sum_contiguous_numbers(input, sum)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            first_rulebreaker(
                &parse_input(
                    "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
",
                ),
                5
            ),
            127
        );
    }

    #[test]
    fn test_part2() {
        let input = &parse_input(
            "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
",
        );
        assert_eq!(
            sum_contiguous_numbers(input, first_rulebreaker(input, 5)),
            62
        )
    }
}
