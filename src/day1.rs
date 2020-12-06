use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|el| el.parse().unwrap()).collect()
}

fn find_double_2020_expenses(expenses: &[u32]) -> [u32; 2] {
    let mut v = expenses.to_vec();
    v.sort_unstable();
    for x in &v {
        for y in v.iter().rev() {
            if x + y == 2020 {
                return [*x, *y];
            }
        }
    }
    unreachable!()
}

fn find_triple_2020_expenses(expenses: &[u32]) -> [u32; 3] {
    let mut v = expenses.to_vec();
    v.sort_unstable();
    for x in &v {
        for y in v.iter().rev() {
            for z in &v {
                if x + y + z == 2020 {
                    return [*x, *y, *z];
                }
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    find_double_2020_expenses(input).iter().product()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    find_triple_2020_expenses(input).iter().product()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            find_double_2020_expenses(&vec![1721, 979, 366, 299, 675, 1456])
                .iter()
                .product::<u32>(),
            514579u32
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            find_triple_2020_expenses(&vec![1721, 979, 366, 299, 675, 1456])
                .iter()
                .product::<u32>(),
            241861950u32
        );
    }
}
