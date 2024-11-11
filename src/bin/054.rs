use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution to [Poker Hands](https://projecteuler.net/problem=54). (Problem 54)
fn main() {
    let mut answer = 0;

    let input_hands = File::open("data/054.txt").expect("should be able to read input file");
    let hands_buffer = BufReader::new(input_hands);

    for line in hands_buffer.lines() {
        let line = line.expect("should be able to read input file line");
        let cards = line.split_whitespace().collect::<Vec<_>>();

        let (player_1_hand_cards, player_2_hand_cards) = cards.split_at(cards.len() / 2);

        let player_1_hand = Hand::new(player_1_hand_cards);
        let player_2_hand = Hand::new(player_2_hand_cards);

        if player_1_hand > player_2_hand {
            answer += 1;
        }
    }

    println!("Answer: {}", answer);
}

#[derive(PartialEq)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(hand: &[&str]) -> Self {
        let mut cards = hand
            .iter()
            .map(|card| {
                let (rank, suit) = card.split_at(1);

                let rank = rank.parse::<u8>().unwrap_or(match rank {
                    "T" => 10,
                    "J" => 11,
                    "Q" => 12,
                    "K" => 13,
                    "A" => 14,
                    _ => 0,
                });

                Card {
                    rank,
                    suit: suit.to_string(),
                }
            })
            .collect::<Vec<_>>();

        cards.sort_by(|a, b| b.rank.cmp(&a.rank));

        Self { cards }
    }

    fn poker_hand(&self) -> PokerHand {
        let ranks_count = self.cards.iter().fold(HashMap::new(), |mut ranks, card| {
            *ranks.entry(card.rank).or_insert(0) += 1;

            ranks
        });

        let suits_count = self
            .cards
            .iter()
            .fold(HashMap::with_capacity(4), |mut suits, card| {
                *suits.entry(&card.suit).or_insert(0) += 1;

                suits
            });

        let is_flush = suits_count.len() == 1;
        let is_straight = ranks_count.len() == self.cards.len()
            && self.cards.windows(2).all(|card_pair| {
                card_pair[0].rank == card_pair[1].rank + 1
                    || (card_pair[0].rank == 14 && card_pair[1].rank == 5)
            });
        let high_card = self
            .cards
            .first()
            .expect("cards should contain at least one card");

        if is_flush && is_straight && high_card.rank == 14 {
            return PokerHand::RoyalFlush;
        } else if is_flush && is_straight {
            return PokerHand::StraightFlush;
        } else if let Some(four_of_a_kind) = Hand::find_by_rank_count(&ranks_count, 4).first() {
            return PokerHand::FourOfAKind(four_of_a_kind.0);
        }

        let three_of_a_kind = Hand::find_by_rank_count(&ranks_count, 3)
            .first()
            .map(|(rank, count)| (*rank, *count));
        let mut pairs = Hand::find_by_rank_count(&ranks_count, 2);

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        if three_of_a_kind.is_some() && pairs.len() > 0 {
            return PokerHand::FullHouse(
                three_of_a_kind.expect("three_of_a_kind should be set").0,
                pairs
                    .first()
                    .expect("pairs should contain at least one pair")
                    .0,
            );
        } else if is_flush {
            return PokerHand::Flush;
        } else if is_straight {
            return PokerHand::Straight;
        } else if let Some(three_of_a_kind) = three_of_a_kind {
            return PokerHand::ThreeOfAKind(three_of_a_kind.0);
        } else if pairs.len() == 2 {
            PokerHand::TwoPairs(pairs[0].0, pairs[1].0)
        } else if let Some(pair) = pairs.first() {
            PokerHand::OnePair(pair.0)
        } else {
            PokerHand::HighCard(high_card.rank)
        }
    }

    fn find_by_rank_count(ranks_count: &HashMap<u8, i32>, expected_count: i32) -> Vec<(u8, i32)> {
        ranks_count
            .iter()
            .filter(|(_, &count)| count == expected_count)
            .map(|(&rank, &count)| (rank, count))
            .collect::<Vec<_>>()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let poker_hand_ordering = self.poker_hand().partial_cmp(&other.poker_hand());

        let mut self_iter = self.cards.iter();
        let mut other_iter = other.cards.iter();

        let mut rank_ordering = Ordering::Equal;
        while let (Some(self_card), Some(other_card)) = (self_iter.next(), other_iter.next()) {
            rank_ordering = self_card.rank.cmp(&other_card.rank);

            if rank_ordering != Ordering::Equal {
                break;
            }
        }

        if poker_hand_ordering != Some(Ordering::Equal) {
            poker_hand_ordering
        } else {
            Some(rank_ordering)
        }
    }
}

#[derive(PartialEq)]
struct Card {
    rank: u8,
    suit: String,
}

#[derive(PartialEq, PartialOrd)]
enum PokerHand {
    HighCard(u8),
    OnePair(u8),
    TwoPairs(u8, u8),
    ThreeOfAKind(u8),
    Straight,
    Flush,
    FullHouse(u8, u8),
    FourOfAKind(u8),
    StraightFlush,
    RoyalFlush,
}
