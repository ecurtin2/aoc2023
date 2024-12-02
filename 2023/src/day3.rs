use std::{char, collections::HashMap, fmt, fmt::Display};

#[derive(Debug)]
struct Number {
    values: Vec<u32>,
    indices: Vec<(i32, i32)>,
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let coords = format!(
            "({}..{}, {})",
            self.indices.first().unwrap().1,
            self.indices.last().unwrap().1,
            self.indices.first().unwrap().0,
        );
        write!(f, "value = {:}, coords = {:}", self.value(), coords)
    }
}

impl Number {
    fn default() -> Number {
        Number {
            values: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn value(&self) -> u32 {
        self.values.iter().fold(0, |acc, d| acc * 10 + d)
    }

    fn empty(&self) -> bool {
        self.values.is_empty()
    }

    fn neighbors(&self) -> Vec<(i32, i32)> {
        let last = self.indices.last().unwrap();
        let first = self.indices.first().unwrap();
        let mut neighbors = Vec::from([
            (first.0 - 1, first.1),     // left
            (first.0 - 1, first.1 + 1), // top left
            (first.0 - 1, first.1 - 1), // bottom left
            (last.0 + 1, last.1),       // right
            (last.0 + 1, last.1 + 1),   // top right
            (last.0 + 1, last.1 - 1),   // bottom right
        ]);
        // Add top and bottom
        for (x, y) in self.indices.iter() {
            neighbors.push((*x, *y + 1)); // top
            neighbors.push((*x, *y - 1)); // bottom
        }
        neighbors
    }
}

fn parse_line(lines: Vec<String>) -> (Vec<Number>, HashMap<(i32, i32), char>) {
    let mut symbols: HashMap<(i32, i32), char> = HashMap::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut current_number = Number::default();
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c != '.' && !c.is_ascii_digit() {
                symbols.insert((x as i32, y as i32), c);
            }
            if c.is_ascii_digit() {
                current_number.values.push(c.to_digit(10).unwrap());
                current_number.indices.push((x as i32, y as i32));
            } else if !current_number.empty() {
                numbers.push(current_number);
                current_number = Number::default();
            }
        }
    }
    (numbers, symbols)
}

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let (numbers, symbols) = parse_line(inputs);
    let part_numbers = numbers
        .iter()
        .filter(|n| n.neighbors().iter().any(|n| symbols.contains_key(n)));
    part_numbers.map(|n| n.value()).sum()
}

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    let (numbers, symbols) = parse_line(inputs);
    let mut symbol_numbers: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
    for number in numbers {
        for coord in number.neighbors() {
            if symbols.contains_key(&coord) {
                let current = symbol_numbers.entry(coord).or_default();
                current.push(number.value());
            }
        }
    }

    symbol_numbers
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum()
}

// Test parse line on Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
#[cfg(test)]
mod tests {
    use super::*;
    // test part 1
    #[test]
    fn test_d3_part_1() {
        let s = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let inputs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(solve_part1(inputs), 4361);
    }
    fn test_d3_part_2() {
        let s = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let inputs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(solve_part1(inputs), 467835);
    }
}
