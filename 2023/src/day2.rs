use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
enum Color {
    Blue = 0,
    Red = 1,
    Green = 2,
}

#[derive(Debug)]
struct Set {
    colors: Vec<(u32, Color)>,
}

impl Set {
    fn power(&self) -> u32 {
        self.colors.iter().map(|c| c.0).product()
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn minimal_required_set(&self) -> Set {
        // For every color, get the maximum number of that color in any set
        let mut color_counts: HashMap<Color, u32> = HashMap::new();
        for set in &self.sets {
            for &(count, color) in &set.colors {
                let current_count = color_counts.entry(color).or_insert(0);
                *current_count = (*current_count).max(count);
            }
        }
        // convert color_counts to Set
        let max_colors: Vec<(u32, Color)> = color_counts
            .iter()
            .map(|(color, count)| (*count, *color))
            .collect();
        Set { colors: max_colors }
    }
}

fn parse_line(line: &str) -> Game {
    let chunks: Vec<&str> = line.split(|c| c == ':' || c == ';').collect();
    let game_id = chunks[0]
        .strip_prefix("Game")
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();
    let parsed_sets: Vec<Set> = chunks[1..]
        .iter()
        .map(|set| {
            let colors: Vec<(u32, Color)> = set
                .split(',')
                .map(|color| {
                    let pairs: Vec<&str> = color.split_whitespace().collect();
                    let count = pairs[0].parse::<u32>().unwrap();
                    let color = match pairs[1] {
                        "blue" => Color::Blue,
                        "red" => Color::Red,
                        "green" => Color::Green,
                        _ => panic!("Unknown color"),
                    };
                    (count, color)
                })
                .collect();
            Set { colors }
        })
        .collect();

    Game {
        id: game_id,
        sets: parsed_sets,
    }
}

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let games: Vec<Game> = inputs.iter().map(|s| parse_line(s)).collect();
    let possible = games.iter().filter(|g| {
        g.sets.iter().all(|s| {
            s.colors.iter().all(|c| match c.1 {
                Color::Red => c.0 <= 12,
                Color::Green => c.0 <= 13,
                Color::Blue => c.0 <= 14,
            })
        })
    });
    possible.map(|g| g.id).sum()
}

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    let games: Vec<Game> = inputs.iter().map(|s| parse_line(s)).collect();
    games.iter().map(|g| g.minimal_required_set().power()).sum()
}

// Test parse line on Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 66 blue; 2 green";
        let game = parse_line(line);
        assert_eq!(game.id, 1);
        assert_eq!(game.sets.len(), 3);
        assert_eq!(game.sets[0].colors.len(), 2);
        assert_eq!(game.sets[0].colors[0].0, 3);
        assert_eq!(game.sets[0].colors[0].1, Color::Blue);
        assert_eq!(game.sets[0].colors[1].0, 4);
        assert_eq!(game.sets[0].colors[1].1, Color::Red);
        assert_eq!(game.sets[1].colors.len(), 3);
        assert_eq!(game.sets[1].colors[0].0, 1);
        assert_eq!(game.sets[1].colors[0].1, Color::Red);
        assert_eq!(game.sets[1].colors[1].0, 2);
        assert_eq!(game.sets[1].colors[1].1, Color::Green);
        assert_eq!(game.sets[1].colors[2].0, 66);
        assert_eq!(game.sets[1].colors[2].1, Color::Blue);
        assert_eq!(game.sets[2].colors.len(), 1);
        assert_eq!(game.sets[2].colors[0].0, 2);
        assert_eq!(game.sets[2].colors[0].1, Color::Green);
    }

    // test part 1
    #[test]
    fn test_d2_part_1() {
        let s = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let inputs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(solve_part1(inputs), 8);
    }

    #[test]
    fn test_d2_part_2() {
        let s = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let inputs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(solve_part2(inputs), 2286);
    }
}
