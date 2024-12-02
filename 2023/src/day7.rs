use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Card {
    Joker = 0,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}

impl Card {
    fn from_char(c: char, j_is_joker: bool) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => {
                if j_is_joker {
                    Card::Joker
                } else {
                    Card::J
                }
            }
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid card: '{}'", c),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl From<[Card; 5]> for HandType {
    fn from(cards: [Card; 5]) -> Self {
        let mut counts: HashMap<Card, u64> = HashMap::new();
        for card in cards.iter() {
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }

        // If we have jokers, remove them from the counts
        // Add the jokers to the counts of the most common card
        let joker_count = counts.remove(&Card::Joker).unwrap_or(0);
        // Edge case 5 jokers
        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        // sort in descending order of count
        let mut counts_vec: Vec<(Card, u64)> = counts.into_iter().collect();
        counts_vec.sort_by(|a, b| b.1.cmp(&a.1));
        counts_vec[0].1 += joker_count;
        match counts_vec.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if counts_vec[0].1 == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if counts_vec[0].1 == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Invalid number of cards"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Hand {
    // Card: an array of length 5 of cards
    cards: [Card; 5],
    hand_type: HandType,
}

impl Hand {
    fn from_str(s: &str, j_is_joker: bool) -> Self {
        let mut cards: [Card; 5] = [Card::Two; 5];
        for (i, c) in s.chars().enumerate() {
            cards[i] = Card::from_char(c, j_is_joker);
        }
        Hand {
            cards,
            hand_type: cards.into(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

fn parse_inputs(inputs: Vec<String>, j_is_joker: bool) -> Vec<(Hand, u64)> {
    inputs
        .iter()
        .map(|s| {
            let hand = Hand::from_str(&s[..5], j_is_joker);
            let bid = s[6..].parse::<u64>().unwrap();
            (hand, bid)
        })
        .collect()
}

pub fn solve_part1(inputs: Vec<String>) -> u64 {
    let mut hands = parse_inputs(inputs, false);
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    let total_winnings: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as u64 + 1) * bid)
        .sum();
    total_winnings
}

pub fn solve_part2(inputs: Vec<String>) -> u64 {
    let mut hands = parse_inputs(inputs, true);
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    let total_winnings: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as u64 + 1) * bid)
        .sum();
    total_winnings
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_d7_p1() {
        let s = r#"32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"#;
        let inputs = s.split('\n').map(|s| s.trim().to_string()).collect();
        assert_eq!(solve_part1(inputs), 6440);
    }

    #[test]
    fn test_d7_p2() {
        let s = r#"32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"#;

        let inputs = s.split('\n').map(|s| s.trim().to_string()).collect();
        assert_eq!(solve_part2(inputs), 5905);
    }
}
