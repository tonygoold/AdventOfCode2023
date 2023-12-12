use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidCard,
    InvalidBid,
    InvalidHand,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Kind {
    pub fn to_string(&self) -> String {
        match *self {
            Kind::HighCard => "High Card",
            Kind::OnePair => "One Pair",
            Kind::TwoPair => "Two Pair",
            Kind::ThreeOfAKind => "Three of a Kind",
            Kind::FullHouse => "Full House",
            Kind::FourOfAKind => "Four of a Kind",
            Kind::FiveOfAKind => "Five of a Kind",
        }.to_owned()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    value: u8,
}

impl Card {
    pub fn to_char(&self) -> char {
        match self.value {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            11 => 'J',
            10 => 'T',
            9 => '9',
            8 => '8',
            7 => '7',
            6 => '6',
            5 => '5',
            4 => '4',
            3 => '3',
            2 => '2',
            1 => 'J',
            _ => panic!("Invalid Card"),
        }
    }

    pub fn try_from(value: char, jokers: bool) -> Result<Self, ParseError> {
        if let Some(value) = value.to_digit(10) {
            return Ok(Card { value: value as u8 });
        }
        match value {
            'A' => Ok(Card { value: 14 }),
            'K' => Ok(Card { value: 13 }),
            'Q' => Ok(Card { value: 12 }),
            'J' => Ok(Card { value: if jokers { 1 } else { 11 } }),
            'T' => Ok(Card { value: 10 }),
            _ => Err(ParseError::InvalidCard),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    kind: Kind,
    bid: usize,
}

impl Hand {
    pub fn from_str(s: &str, jokers: bool) -> Result<Self, ParseError> {
        let mut parts = s.split_ascii_whitespace();
        let hand = parts.next().ok_or(ParseError::InvalidHand)?;
        let bid = parts.next().ok_or(ParseError::InvalidHand)?
            .parse::<usize>().map_err(|_| ParseError::InvalidBid)?;
        if hand.len() != 5 {
            return Err(ParseError::InvalidHand);
        }
        let mut cs = hand.chars();
        let cards = [
            Card::try_from(cs.next().unwrap(), jokers)?,
            Card::try_from(cs.next().unwrap(), jokers)?,
            Card::try_from(cs.next().unwrap(), jokers)?,
            Card::try_from(cs.next().unwrap(), jokers)?,
            Card::try_from(cs.next().unwrap(), jokers)?,
        ];
        let kind = Hand::kind_for_hand(&cards, jokers);
        Ok(Hand { cards, kind, bid })
    }

    pub fn kind_for_hand(hand: &[Card; 5], jokers: bool) -> Kind {
        // Not the most efficient
        let mut distinct: HashMap<u8, u8> = HashMap::new();
        let mut joker_count: u8 = 0;
        for c in hand {
            if jokers && c.value == 1 {
                joker_count += 1;
            } else {
                distinct.entry(c.value).or_default().add_assign(1);
            }
        }
        if joker_count == 5 {
            return Kind::FiveOfAKind;
        }
        for (_, val) in distinct.iter_mut() {
            *val += joker_count;
        }
        let mut counts: Vec<_> = distinct.values().collect();
        counts.sort_unstable();
        let count = counts.len();
        let last = counts.pop().unwrap_or(&5);
        match count {
            1 => Kind::FiveOfAKind,
            2 => if *last == 4 {
                Kind::FourOfAKind
            } else {
                Kind::FullHouse
            },
            3 => if *last == 3 {
                Kind::ThreeOfAKind
            } else {
                Kind::TwoPair
            },
            4 => Kind::OnePair,
            _ => Kind::HighCard,
        }
    }

    pub fn cards(&self) -> &[Card; 5] {
        &self.cards
    }

    pub fn bid(&self) -> usize {
        self.bid
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn to_string(&self) -> String {
        format!("{}{}{}{}{} ({}) {}",
            self.cards[0].to_char(),
            self.cards[1].to_char(),
            self.cards[2].to_char(),
            self.cards[3].to_char(),
            self.cards[4].to_char(),
            self.kind.to_string(),
            self.bid
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::{Hand, Kind};

    #[test]
    pub fn test_kinds_jokers() {
        let h1 = Hand::from_str("32T3K 0", true).unwrap();
        let h2 = Hand::from_str("T55J5 0", true).unwrap();
        let h3 = Hand::from_str("KK677 0", true).unwrap();
        let h4 = Hand::from_str("KTJJT 0", true).unwrap();
        let h5 = Hand::from_str("QQQJA 0", true).unwrap();
        assert_eq!(Kind::OnePair, h1.kind());
        assert_eq!(Kind::FourOfAKind, h2.kind());
        assert_eq!(Kind::TwoPair, h3.kind());
        assert_eq!(Kind::FourOfAKind, h4.kind());
        assert_eq!(Kind::FourOfAKind, h5.kind());
    }

    #[test]
    pub fn test_all_jokers() {
        let h1 = Hand::from_str("JJJJJ 0", true).unwrap();
        assert_eq!(Kind::FiveOfAKind, h1.kind());
    }
}