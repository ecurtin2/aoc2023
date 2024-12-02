use nom::{
    character::complete::{i32 as nom_i32, multispace1},
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;
use std::iter::zip;

fn parse_i32_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(nom_i32, multispace1, nom_i32)(input)
}

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) =
        inputs.iter().map(|s| parse_i32_pair(s).unwrap().1).unzip();
    left.sort();
    right.sort();
    zip(left, right).map(|(l, r)| (r - l).abs()).sum::<i32>() as u32
}

fn value_counts(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();
    for i in v {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }
    counts
}

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    let (left, right): (Vec<i32>, Vec<i32>) =
        inputs.iter().map(|s| parse_i32_pair(s).unwrap().1).unzip();

    let right_counts = value_counts(right);
    left.iter()
        .map(|l| l * right_counts.get(l).unwrap_or(&0))
        .sum::<i32>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let inputs: Vec<String> = Vec::from(["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"])
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(solve_part1(inputs), 11);
    }

    #[test]
    fn test_part_2() {
        let inputs: Vec<String> = Vec::from(["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"])
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(solve_part2(inputs), 31);
    }
}
