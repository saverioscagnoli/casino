use crate::database::User;

use super::deck::{Deck, DeckFactory, FrenchCard};

pub struct BlackjackPlayer {
    user: Option<User>,
    hand: Vec<FrenchCard>,
    bet: Option<u32>,
    is_dealer: bool,
}

impl BlackjackPlayer {
    pub fn player(user: User, bet: u32) -> Self {
        Self {
            user: Some(user),
            hand: Vec::new(),
            bet: Some(bet),
            is_dealer: false,
        }
    }

    pub fn dealer() -> Self {
        Self {
            user: None,
            hand: Vec::new(),
            bet: None,
            is_dealer: true,
        }
    }
}

pub struct Blackjack {
    players: Vec<BlackjackPlayer>,
    dealer: BlackjackPlayer,
    deck: Deck<FrenchCard>,
}

impl Blackjack {
    pub fn new() -> Self {
        let deck = DeckFactory::french();

        let dealer = BlackjackPlayer::dealer();
        let players = Vec::new();

        Self {
            players,
            dealer,
            deck,
        }
    }
}
