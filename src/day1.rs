fn parse_to_int(s: &str, allow_names: bool) -> Option<u32> {
    let mut checks: Vec<&str> = Vec::from(["1", "2", "3", "4", "5", "6", "7", "8", "9"]);
    if allow_names {
        checks.extend([
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]);
    }

    for &check in checks.iter() {
        if s.starts_with(check) {
            match check {
                "one" => return Some(1),
                "two" => return Some(2),
                "three" => return Some(3),
                "four" => return Some(4),
                "five" => return Some(5),
                "six" => return Some(6),
                "seven" => return Some(7),
                "eight" => return Some(8),
                "nine" => return Some(9),
                _ => return check.parse::<u32>().ok(),
            }
        }
    }
    None
}

fn get_calibration_value(s: &str, allow_names: bool) -> u32 {
    let first: Option<u32> = s
        .char_indices()
        .find_map(|(i, _)| parse_to_int(&s[i..], allow_names));

    let last: Option<u32> = s
        .char_indices()
        .rev()
        .find_map(|(i, _)| parse_to_int(&s[i..], allow_names));
    match (first, last) {
        (Some(f), Some(l)) => f * 10 + l,
        _ => panic!("No first or last int found"),
    }
}

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    inputs.iter().map(|s| get_calibration_value(s, false)).sum()
}

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    inputs.iter().map(|s| get_calibration_value(s, true)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_value() {
        assert_eq!(get_calibration_value("1abc2", false), 12);
    }

    #[test]
    fn test_calibration_value_2() {
        assert_eq!(get_calibration_value("pqr3stu8vwx", false), 38);
    }

    #[test]
    fn test_calibration_value_3() {
        assert_eq!(get_calibration_value("a1b2c3d4e5f", false), 15);
    }

    #[test]
    fn test_calibration_value_4() {
        assert_eq!(get_calibration_value("treb7uchet", false), 77);
    }

    #[test]
    fn test_part_1() {
        let inputs: Vec<String> = Vec::from(["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"])
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(solve_part1(inputs), 142);
    }

    #[test]
    fn test_part_2() {
        let inputs: Vec<String> = Vec::from([
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ])
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(solve_part2(inputs), 281);
    }
}
