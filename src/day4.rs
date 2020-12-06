use anyhow::Result;
use serde::Deserialize;

use aoc_runner_derive::{aoc, aoc_generator};

const VALID_EYE_COLOURS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Debug, Deserialize)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn validate_byr(&self) -> Result<bool> {
        Ok(self.byr.parse::<u16>().map(|x| x >= 1920 && x <= 2002)?)
    }

    fn validate_iyr(&self) -> Result<bool> {
        Ok(self.iyr.parse::<u16>().map(|x| x >= 2010 && x <= 2020)?)
    }

    fn validate_eyr(&self) -> Result<bool> {
        Ok(self.eyr.parse::<u16>().map(|x| x >= 2020 && x <= 2030)?)
    }

    fn validate_hgt(&self) -> bool {
        match self.hgt.get((self.hgt.len() - 2)..) {
            Some("in") => self
                .hgt
                .get(..2)
                .unwrap()
                .parse::<u16>()
                .map_or(false, |height| height >= 59 && height <= 76),
            Some("cm") => self
                .hgt
                .get(..3)
                .unwrap()
                .parse::<u16>()
                .map_or(false, |height| height >= 150 && height <= 193),
            _ => false,
        }
    }

    fn validate_hcl(&self) -> bool {
        self.hcl.starts_with('#')
            && self
                .hcl
                .chars()
                .skip(1)
                .all(|c| (c.is_lowercase() || c.is_numeric()) && c.is_ascii_hexdigit())
    }

    fn validate_ecl(&self) -> bool {
        VALID_EYE_COLOURS.contains(&self.ecl.as_str())
    }

    fn validate_pid(&self) -> bool {
        self.pid.parse::<u32>().is_ok() && self.pid.len() == 9
    }

    fn validate(&self) -> Result<bool> {
        Ok(self.validate_byr()?
            && self.validate_iyr()?
            && self.validate_eyr()?
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid())
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Result<Passport>> {
    input
        .split("\n\n")
        .map(|el| {
            // The passports look sorta kinda like YAML if you hack it around a bit?
            // Saves implementing a bunch of parsing logic.
            Ok(serde_yaml::from_str(&format!(
                "{}\"",
                &el.replace('\n', "\"\n")
                    .replace(' ', "\"\n")
                    .replace(':', ": \"")
            ))?)
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Result<Passport>]) -> u32 {
    input.iter().map(|x| if x.is_ok() { 1 } else { 0 }).sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Result<Passport>]) -> u32 {
    input
        .iter()
        .filter_map(|x| {
            x.as_ref()
                .ok()
                .map(|passport| passport.validate().map_or(0, |x| if x { 1 } else { 0 }))
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
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"
            )),
            2
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input(
                "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            )),
            4
        );
    }
}
