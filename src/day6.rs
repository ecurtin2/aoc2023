#[derive(Debug)]
struct Race {
    time: f64,
    distance: f64,
}

fn parse_lines(inputs: Vec<String>) -> Vec<Race> {
    let times: Vec<f64> = inputs[0]
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect();
    let distances: Vec<f64> = inputs[1]
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect();

    let races: Vec<Race> = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();
    races
}

// x * (t - x) > d
// x * t - x^2 > d
// x^2 - x * t + d < 0
fn parabola_zeros(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = (b.powf(2.0) - 4.0 * a * c).sqrt();
    let x1: f64 = (-b + discriminant) / (2.0 * a);
    let x2: f64 = (-b - discriminant) / (2.0 * a);
    let larger = x1.max(x2);
    let smaller = x1.min(x2);
    (smaller, larger)
}

fn n_wins(time: f64, distance: f64) -> u64 {
    let (x1, x2) = parabola_zeros(1.0, -time, distance);
    let x1c = x1.ceil();
    let x2f = x2.floor();
    let d1 = x1c * (time - x1);
    let d2 = x2f * (time - x2);

    // Check the edge case where the zeros are ties - they are not wins
    let mut n_ties: u64 = 0;
    if (distance - d1).abs() < 1e-6 {
        n_ties += 1;
    }
    if (distance - d2).abs() < 1e-6 {
        n_ties += 1;
    }
    // +1 because edges are inclusive
    (x2f - x1c) as u64 - n_ties + 1
}

pub fn solve_part1(inputs: Vec<String>) -> u64 {
    let races = parse_lines(inputs);
    races.iter().map(|r| n_wins(r.time, r.distance)).product()
}

pub fn solve_part2(inputs: Vec<String>) -> u64 {
    let time: f64 = inputs[0]
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let dist: f64 = inputs[1]
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    n_wins(time, dist)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_d6_p1() {
        let s = r#"Time:      7  15   30
Distance:  9  40  200
        "#;
        let inputs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(solve_part1(inputs), 288);
    }

    #[test]
    fn test_d6_p2() {
        let s = r#"Time:      7  15   30
Distance:  9  40  200
        "#;
        let inputs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(solve_part2(inputs), 71503);
    }
}
