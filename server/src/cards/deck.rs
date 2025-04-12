use std::{collections::VecDeque, fmt::Display};

use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::math::{Pick, Shuffle};

pub enum CardKind {
    French,
    Italian,
}

#[derive(Debug, Clone, Copy, EnumIter, Serialize)]
pub enum FrenchSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Display for FrenchSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrenchSuit::Hearts => write!(f, "Hearts"),
            FrenchSuit::Diamonds => write!(f, "Diamonds"),
            FrenchSuit::Clubs => write!(f, "Clubs"),
            FrenchSuit::Spades => write!(f, "Spades"),
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, Serialize)]
pub enum FrenchRank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for FrenchRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrenchRank::Ace => write!(f, "Ace"),
            FrenchRank::Two => write!(f, "Two"),
            FrenchRank::Three => write!(f, "Three"),
            FrenchRank::Four => write!(f, "Four"),
            FrenchRank::Five => write!(f, "Five"),
            FrenchRank::Six => write!(f, "Six"),
            FrenchRank::Seven => write!(f, "Seven"),
            FrenchRank::Eight => write!(f, "Eight"),
            FrenchRank::Nine => write!(f, "Nine"),
            FrenchRank::Ten => write!(f, "Ten"),
            FrenchRank::Jack => write!(f, "Jack"),
            FrenchRank::Queen => write!(f, "Queen"),
            FrenchRank::King => write!(f, "King"),
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, Serialize)]
pub enum ItalianSuit {
    Cups,
    Coins,
    Swords,
    Clubs,
}

impl Display for ItalianSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItalianSuit::Cups => write!(f, "Cups"),
            ItalianSuit::Coins => write!(f, "Coins"),
            ItalianSuit::Swords => write!(f, "Swords"),
            ItalianSuit::Clubs => write!(f, "Clubs"),
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, Serialize)]
pub enum ItalianRank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Knave,
    King,
}

impl Display for ItalianRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItalianRank::Ace => write!(f, "Ace"),
            ItalianRank::Two => write!(f, "Two"),
            ItalianRank::Three => write!(f, "Three"),
            ItalianRank::Four => write!(f, "Four"),
            ItalianRank::Five => write!(f, "Five"),
            ItalianRank::Six => write!(f, "Six"),
            ItalianRank::Seven => write!(f, "Seven"),
            ItalianRank::Eight => write!(f, "Eight"),
            ItalianRank::Nine => write!(f, "Nine"),
            ItalianRank::Jack => write!(f, "Jack"),
            ItalianRank::Knave => write!(f, "Knave"),
            ItalianRank::King => write!(f, "King"),
        }
    }
}

pub enum SuitKind {
    French(FrenchSuit),
    Italian(ItalianSuit),
}

pub trait Card {
    fn name(&self) -> String;
    fn kind(&self) -> CardKind;
    fn suit(&self) -> SuitKind;
}

#[derive(Debug, Clone, Serialize)]
pub struct FrenchCard {
    pub suit: FrenchSuit,
    pub rank: FrenchRank,
}

impl Card for FrenchCard {
    fn name(&self) -> String {
        format!("{} of {}", self.rank, self.suit)
    }

    fn kind(&self) -> CardKind {
        CardKind::French
    }

    fn suit(&self) -> SuitKind {
        SuitKind::French(self.suit)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ItalianCard {
    pub suit: ItalianSuit,
    pub rank: ItalianRank,
}

impl Card for ItalianCard {
    fn name(&self) -> String {
        format!("{} of {}", self.rank, self.suit)
    }

    fn kind(&self) -> CardKind {
        CardKind::Italian
    }

    fn suit(&self) -> SuitKind {
        SuitKind::Italian(self.suit)
    }
}

pub struct Deck<C: Card> {
    cards: VecDeque<C>,
}

impl<C: Card> Pick<C> for Deck<C> {
    fn pick(&self) -> &C {
        self.cards.pick()
    }

    fn pick_mut(&mut self) -> &mut C {
        self.cards.pick_mut()
    }
}

impl<C: Card> Shuffle for Deck<C> {
    fn shuffle(&mut self) {
        self.cards.shuffle()
    }
}

impl<C: Card> Deck<C> {
    pub fn new() -> Self {
        Self {
            cards: VecDeque::new(),
        }
    }

    pub fn add_card(&mut self, card: C) {
        self.cards.push_back(card);
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
}

pub struct DeckFactory;

impl DeckFactory {
    /// Creates a new deck of cards.
    /// The deck is shuffled and ready to be used.
    pub fn french() -> Deck<FrenchCard> {
        let mut deck = Deck::new();

        for suit in FrenchSuit::iter() {
            for rank in FrenchRank::iter() {
                deck.add_card(FrenchCard { suit, rank });
            }
        }

        deck.shuffle();
        deck
    }

    /// Creates a new deck of cards.
    /// The deck is shuffled and ready to be used.
    pub fn italian() -> Deck<ItalianCard> {
        let mut deck = Deck::new();

        for suit in ItalianSuit::iter() {
            for rank in ItalianRank::iter() {
                deck.add_card(ItalianCard { suit, rank });
            }
        }

        deck.shuffle();
        deck
    }
}
