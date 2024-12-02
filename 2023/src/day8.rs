use nom::{
    bytes::complete::tag, character::complete::alphanumeric1, character::complete::multispace0, IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left = 0,
    Right = 1,
}
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

fn parse_line(line: &str) -> IResult<&str, (String, (String, String))> {
    let (i, key) = alphanumeric1(line)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("(")(i)?;
    let (i, left) = alphanumeric1(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, right) = alphanumeric1(i)?;
    Ok((i, (key.to_string(), (left.to_string(), right.to_string()))))
}

pub fn solve_part1(inputs: Vec<String>) -> u64 {
    let directions: Vec<Direction> = inputs[0].chars().map(|c| c.into()).collect();

    let map: HashMap<String, (String, String)> = inputs[2..]
        .iter()
        .map(|l| parse_line(l).unwrap().1)
        .collect();

    let mut loc: String = "AAA".to_string();
    let dest: String = "ZZZ".to_string();
    for (i, direction) in directions.iter().cycle().enumerate() {
        if loc == dest {
            return i as u64;
        }
        loc = match direction {
            Direction::Left => map.get(&loc).unwrap().0.to_string(),
            Direction::Right => map.get(&loc).unwrap().1.to_string(),
        };
    }
    return 0;
}

pub fn solve_part2(inputs: Vec<String>) -> u64 {
    let directions: Vec<Direction> = inputs[0].chars().map(|c| c.into()).collect();

    let map: HashMap<String, (String, String)> = inputs[2..]
        .iter()
        .map(|l| parse_line(l).unwrap().1)
        .collect();

    let mut walkers: Vec<String> = map.keys().filter(|k| k.ends_with('A')).cloned().collect();
    println!("Walkers: {:?}", walkers);

    for (i, direction) in directions.iter().cycle().enumerate() {
        // Check if all walkers are equal to dest
        if walkers.iter().all(|w| w.ends_with('Z')) {
            return i as u64;
        }
        // Update walkers
        for w in walkers.iter_mut() {
            let new_loc = match direction {
                Direction::Left => map.get(w).unwrap().0.to_string(),
                Direction::Right => map.get(w).unwrap().1.to_string(),
            };
            // Check if walkers are stuck
            if new_loc == *w {
                println!("Stuck at {}", w);
                return i as u64;
            }
            *w = new_loc;
        }
        // Print every 100k steps
        if i % 1000000 == 0 {
            println!("Step: {}", i);
        }


    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_d8_p1() {
        let s = r#"RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"#;
        let inputs = s.split('\n').map(|s| s.trim().to_string()).collect();
        assert_eq!(solve_part1(inputs), 2);

        let s = r#"LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"#;
        let inputs = s.split('\n').map(|s| s.trim().to_string()).collect();
        assert_eq!(solve_part1(inputs), 6);
    }

    #[test]
    fn test_d8_p2() {
        let s = r#"LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"#;

        let inputs = s.split('\n').map(|s| s.trim().to_string()).collect();
        assert_eq!(solve_part2(inputs), 6);
    }
}
