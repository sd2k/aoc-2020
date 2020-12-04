use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[u32]) -> u32 {
    todo!()
}

#[aoc(day2, part2)]
fn part2(input: &[u32]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(514579u32);
    }

    #[test]
    fn test_part2() {
        assert_eq!(241861950u32);
    }
}
