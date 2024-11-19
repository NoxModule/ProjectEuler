mod hand;

pub use hand::Hand;

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub rank: u8,
    pub suit: String,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum PokerHand {
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
