use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

use aoc_runner_derive::{aoc, aoc_generator};

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<colour>.+) bags contain (?P<contents>.+)+.").unwrap());

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
struct BagColour(String);

#[derive(Debug)]
struct Bag {
    colour: BagColour,
    capacity: Option<Vec<BagCapacity>>,
}

impl Bag {
    fn total_bags(&self, bags: &HashMap<BagColour, Bag>) -> usize {
        // One for this bag, plus either 0 (if we don't hold any bags), or...
        1 + self.capacity.as_ref().map_or(0, |inner| {
            inner
                .iter()
                // ...for each bag inside, however many of that bag the current
                // bag can hold, multiplied by that bag's capacity.
                .map(|b| b.capacity * bags.get(&b.colour).unwrap().total_bags(&bags))
                .sum::<usize>()
        })
    }
}

impl FromStr for Bag {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        RE.captures(s)
            .with_context(|| format!("No matches for line! {}", s))
            .and_then(|caps| {
                let colour = caps
                    .name("colour")
                    .context("No colour found")?
                    .as_str()
                    .to_string();
                let raw_contents = caps.name("contents").context("No contents found")?.as_str();
                let contents = if raw_contents == "no other bags" {
                    None
                } else {
                    Some(
                        raw_contents
                            .split(", ")
                            .map(str::parse)
                            .collect::<Result<Vec<_>>>()?,
                    )
                };
                Ok(Self {
                    colour: BagColour(colour),
                    capacity: contents,
                })
            })
    }
}

#[derive(Clone, Debug, Deserialize)]
struct BagCapacity {
    capacity: usize,
    colour: BagColour,
}

impl FromStr for BagCapacity {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.splitn(2, ' ');
        let capacity = iter
            .next()
            .context("Invalid capacity")
            .and_then(|x| x.parse().context("Invalid capacity"))?;
        let mut right_iter = iter
            .next()
            .context("Invalid capacity")?
            .trim_end_matches('s')
            .rsplitn(2, " bag");
        let colour = BagColour(right_iter.nth(1).context("Invalid capacity")?.to_string());
        Ok(Self { capacity, colour })
    }
}

fn can_contain_bag<'a>(
    desired: &'a BagColour,
    current: &'a BagColour,
    bags: &'a HashMap<BagColour, Bag>,
    mut cache: &mut HashMap<&'a BagColour, bool>,
) -> bool {
    // Can't use `entry` or `raw_entry` APIs because we need to pass a mutable
    // reference to the cache to this function, which wouldn't be allowed
    // if we were currently holding such a reference.
    if let Some(cached) = cache.get(current) {
        *cached
    } else {
        let res = bags
            .get(current)
            .and_then(|bag| bag.capacity.as_ref())
            .map_or(false, |inner| {
                inner.iter().any(|b| {
                    &b.colour == desired || can_contain_bag(desired, &b.colour, bags, &mut cache)
                })
            });
        cache.insert(current, res);
        res
    }
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> HashMap<BagColour, Bag> {
    input
        .lines()
        .map(|el| {
            let bag: Bag = el.parse().unwrap();
            (bag.colour.clone(), bag)
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<BagColour, Bag>) -> usize {
    let desired = BagColour("shiny gold".to_string());
    let mut cache = HashMap::new();
    input
        .keys()
        .filter(|k| can_contain_bag(&desired, k, input, &mut cache))
        .count()
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<BagColour, Bag>) -> usize {
    let desired = BagColour("shiny gold".to_string());
    input.get(&desired).unwrap().total_bags(&input) - 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse_input(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            )),
            4
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            )),
            32
        );
        assert_eq!(
            part2(&parse_input(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"
            )),
            126
        );
    }
}
