use std::{cmp::Ordering, collections::HashMap};

use super::{Card, PokerHand};

#[derive(Clone)]
pub struct Hand {
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

        // calculate common poker hands
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

        // calculate more common poker hands
        let three_of_a_kind = Hand::find_by_rank_count(&ranks_count, 3)
            .first()
            .map(|(rank, count)| (*rank, *count));
        let mut pairs = Hand::find_by_rank_count(&ranks_count, 2);

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        if three_of_a_kind.is_some() && pairs.len() > 0 {
            PokerHand::FullHouse(
                three_of_a_kind.expect("three_of_a_kind should be set").0,
                pairs
                    .first()
                    .expect("pairs should contain at least one pair")
                    .0,
            )
        } else if is_flush {
            PokerHand::Flush
        } else if is_straight {
            PokerHand::Straight
        } else if let Some(three_of_a_kind) = three_of_a_kind {
            PokerHand::ThreeOfAKind(three_of_a_kind.0)
        } else if pairs.len() == 2 {
            PokerHand::TwoPairs(pairs[1].0, pairs[0].0)
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

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards
            .iter()
            .zip(other.cards.iter())
            .all(|(self_card, other_card)| self_card.rank == other_card.rank)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let poker_hand_ordering = self.poker_hand().partial_cmp(&other.poker_hand());
        if poker_hand_ordering != Some(Ordering::Equal) {
            return poker_hand_ordering;
        }

        self.cards
            .iter()
            .zip(other.cards.iter())
            .find_map(|(self_card, other_card)| {
                let rank_ordering = self_card.rank.cmp(&other_card.rank);

                if rank_ordering == Ordering::Equal {
                    None
                } else {
                    Some(rank_ordering)
                }
            })
            .or(Some(Ordering::Equal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_returns_expected_sorted_hand_by_rank_in_descending_order() {
        // arrange
        let expected_hand_cards = vec![
            Card {
                rank: 13,
                suit: String::from("D"),
            },
            Card {
                rank: 7,
                suit: String::from("S"),
            },
            Card {
                rank: 6,
                suit: String::from("S"),
            },
            Card {
                rank: 5,
                suit: String::from("H"),
            },
            Card {
                rank: 5,
                suit: String::from("C"),
            },
        ];

        // action
        let actual_hand = Hand::new(&["7S", "5H", "KD", "6S", "5C"]);

        // assert
        assert_eq!(expected_hand_cards, actual_hand.cards);
    }

    #[test]
    fn poker_hand_returns_expected_poker_hand() {
        // action
        let royal_flush_hand = Hand::new(&["AD", "JD", "KD", "QD", "TD"]).poker_hand();
        let straight_flush_hand = Hand::new(&["4S", "2S", "5S", "3S", "6S"]).poker_hand();
        let four_of_a_kind_hand = Hand::new(&["AS", "AD", "AH", "AC", "7D"]).poker_hand();
        let full_house_hand = Hand::new(&["3D", "3S", "3C", "4D", "4C"]).poker_hand();
        let flush_hand = Hand::new(&["KS", "2S", "5S", "3S", "JS"]).poker_hand();
        let low_straight_hand = Hand::new(&["AS", "2H", "5H", "3D", "4C"]).poker_hand();
        let high_straight_hand = Hand::new(&["AS", "JD", "QS", "TC", "KS"]).poker_hand();
        let three_of_a_kind_hand = Hand::new(&["KH", "2D", "4C", "4D", "4S"]).poker_hand();
        let two_pairs_hand = Hand::new(&["TQ", "KC", "TS", "8C", "KH"]).poker_hand();
        let one_pair_hand = Hand::new(&["3D", "QD", "7H", "8D", "QS"]).poker_hand();
        let high_card_hand = Hand::new(&["5D", "8C", "9S", "JS", "AC"]).poker_hand();

        // assert
        assert_eq!(PokerHand::RoyalFlush, royal_flush_hand);
        assert_eq!(PokerHand::StraightFlush, straight_flush_hand);
        assert_eq!(PokerHand::FourOfAKind(14), four_of_a_kind_hand);
        assert_eq!(PokerHand::FullHouse(3, 4), full_house_hand);
        assert_eq!(PokerHand::Flush, flush_hand);
        assert_eq!(PokerHand::Straight, low_straight_hand);
        assert_eq!(PokerHand::Straight, high_straight_hand);
        assert_eq!(PokerHand::ThreeOfAKind(4), three_of_a_kind_hand);
        assert_eq!(PokerHand::TwoPairs(13, 10), two_pairs_hand);
        assert_eq!(PokerHand::OnePair(12), one_pair_hand);
        assert_eq!(PokerHand::HighCard(14), high_card_hand);
    }

    #[test]
    fn partial_cmp_determines_poker_hand_ordering_without_parameters() {
        // arrange
        let royal_flush_hand = Hand::new(&["AD", "JD", "KD", "QD", "TD"]);
        let flush_hand = Hand::new(&["3S", "7S", "QS", "2S", "6S"]);

        // assert
        assert!(royal_flush_hand > flush_hand);
    }

    #[test]
    fn partial_cmp_determines_poker_hand_ordering_with_parameters() {
        // arrange
        let low_two_pairs_hand = Hand::new(&["9H", "KD", "9D", "2H", "KS"]);
        let high_two_pairs_hand = Hand::new(&["JS", "KC", "JQ", "4C", "KH"]);

        // assert
        assert!(low_two_pairs_hand < high_two_pairs_hand);
    }

    #[test]
    fn partial_cmp_determines_poker_hand_ordering_using_high_card() {
        // arrange
        let low_high_card_hand = Hand::new(&["5H", "4C", "8D", "JD", "AC"]);
        let high_high_card_hand = Hand::new(&["5C", "8S", "9C", "JH", "AS"]);

        // assert
        assert!(low_high_card_hand < high_high_card_hand);
    }

    #[test]
    fn partial_cmp_determines_poker_hand_equality() {
        // arrange
        let two_pairs_hand = Hand::new(&["9Q", "KC", "9S", "8C", "KH"]);

        // assert
        assert!(two_pairs_hand.clone() == two_pairs_hand)
    }
}
