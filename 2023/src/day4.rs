use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::multispace0,
    character::complete::multispace1, multi::separated_list1, IResult,
};

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn my_winning_numbers(&self) -> Vec<u32> {
        self.winning_numbers
            .iter()
            .filter(|n| self.my_numbers.contains(n))
            .copied()
            .collect()
    }
    fn points(&self) -> u32 {
        let len = self.my_winning_numbers().len();
        if len == 0 {
            0
        } else {
            2u32.pow(self.my_winning_numbers().len() as u32 - 1)
        }
    }
}

fn parse_line(line: &str) -> IResult<&str, Card> {
    let (i, _) = tag("Card")(line)?;
    let (i, _) = multispace0(i)?;
    let (i, _card_id) = digit1(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, winning_numbers) = separated_list1(multispace1, digit1)(i)?;
    let (i, _) = multispace0(i)?;
    let winning_numbers: Vec<u32> = winning_numbers.iter().map(|s| s.parse().unwrap()).collect();
    let (i, _) = tag("|")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, my_numbers) = separated_list1(multispace1, digit1)(i)?;
    let my_numbers: Vec<u32> = my_numbers.iter().map(|s| s.parse().unwrap()).collect();

    Ok((
        i,
        Card {
            winning_numbers,
            my_numbers,
        },
    ))
}

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let cards: Vec<Card> = inputs.iter().map(|l| parse_line(l).unwrap().1).collect();
    cards.iter().map(|c| c.points()).sum()
}

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    let cards: Vec<Card> = inputs.iter().map(|l| parse_line(l).unwrap().1).collect();
    let mut copies: Vec<u32> = vec![1; cards.len()];
    let mut result: u32 = 0;

    for (current_i, card) in cards.iter().enumerate() {
        let n_winners = card.my_winning_numbers().len();
        for i in 0..n_winners {
            copies[current_i + i + 1] += copies[current_i]
        }
        result += copies[current_i];
    }
    result
}

// Test parse line on Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_d4_p1() {
        let s = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let inputs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(solve_part1(inputs), 13);
    }
    #[test]
    fn test_d4_p2() {
        let s = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        let inputs = s.split('\n').map(|s| s.to_string()).collect();
        assert_eq!(solve_part2(inputs), 30);
    }
}
