pub mod cards;
pub mod rulesets;
pub mod state;

use crate::table::cards::*;
use crate::table::rulesets::*;
use crate::table::state::*;

use rand;
use rand::seq::SliceRandom;

use std::collections::*;
use strum::IntoEnumIterator;

pub struct Game {
    deck: HashMap<Card, u8>,           // Location of the cards
    starting_hands: HashMap<Card, u8>, // Location of the cards at game start
    // Cards in the deck can be in the following locations:
    // 0. In Play
    // 1. Hand Player1  2. Hand Player2  3. Hand Player3  4. Hand Player4
    // 5. Owned Player1 6.Owned Player2 7. Owned Player3 8. Owned Player4
    pub ruleset: Option<Ruleset>,
    // Tracks the last game announced
    teams: Option<u8>,
    // Only two teams can exist
    // Teams might not be known to all players at the start
    // Bits 0..3 descibe team assignment for players 1 to 4
    pub winner: Option<u8>,
    // Winner 0..3
    pub game_progress: GameState,
    pub announce_turn: u8,
    pub play_turn: u8,
    pub vorhand: u8,
    pub first_card: Option<Card>,
    /*
    The game progresses by calling:
    - new() to initialize a game
    - Players announce_game() until game_progress changes to 1 which means a ruleset is chosen
    - Players play_card() clockwise until the winner is chosen
    */
}
impl Game {
    pub fn new(dealer: u8) -> Self {
        let starting_cards = Self::new_cards();
        Self {
            deck: starting_cards.clone(),
            starting_hands: starting_cards,
            ruleset: None,
            teams: None,
            winner: None,
            game_progress: GameState::new(),
            announce_turn: (dealer + 1) % 4,
            play_turn: (dealer + 1) % 4,
            // Tracks the first player to come out in game phase and the last announcer in announcement phase
            vorhand: (dealer + 1) % 4,
            first_card: None,
        }
    }

    pub fn announce_game(&mut self, announce_ruleset: Option<Ruleset>) -> bool {
        match announce_ruleset {
            Some(announced_ruleset) => {
                if self.announcement_is_valid(announced_ruleset) {
                    self.ruleset = Some(announced_ruleset);
                    self.game_progress.AnnounceSome();
                } else {
                    return false;
                }
            }
            None => {
                self.announce_turn = self.announce_turn + 1;
                self.game_progress.AnnounceNone();
            }
        }
        self.announce_turn = self.announce_turn + 1 % 4;
        true
    }

    pub fn announcement_is_valid(&self, announce_ruleset: Ruleset) -> bool {
        // Announcement needs to be higher value than the last
        // TODO: Add more rulesets
        if announce_ruleset.value <= self.ruleset.map_or(0, |x| x.value) {
            return false;
        }

        let announce_sow = announce_ruleset.sow;
        if announce_sow.is_some() {
            // Caller cannot have the sow in hand that is called
            if self.get_card_owner(&announce_sow.unwrap()) == Ok(self.announce_turn) {
                return false;
            }
            // Caller needs to have at least one card of the sow color which is not trump
            // TODO: check for not being trump
            if !self.has_number_of_color_in_hand(announce_sow.unwrap().color, self.announce_turn) {
                return false;
            }
        }
        true
    }

    pub fn get_cards_in_location(&self, location: u8) -> HashSet<Card> {
        let mut hand = HashSet::new();
        for card in self.deck.clone() {
            if card.1 == location {
                hand.insert(card.0);
            }
        }
        hand
    }

    fn new_cards() -> HashMap<Card, u8> {
        let mut deckvec = Vec::new();
        for color in Colors::iter() {
            for symbol in Symbols::iter() {
                deckvec.push(Card { symbol, color });
            }
        }
        deckvec.shuffle(&mut rand::thread_rng());

        let mut deck = HashMap::new();
        for (n, card) in deckvec.iter().enumerate() {
            deck.insert(*card, (n % 4 + 1).try_into().unwrap());
        }
        deck
    }

    fn get_card_owner(&self, card: &Card) -> Result<u8, &'static str> {
        match self.starting_hands.get(card) {
            Some(location) => Ok(location - 1),
            None => Err("Card has disappeared"),
        }
    }

    fn card_is_valid(&self, card: &Card) -> Result<bool, &'static str> {
        if self.ruleset.is_none() {
            return Err("No ruleset has been chosen");
        }
        // Is in player hand whos turn it is
        if Ok(self.vorhand) != self.get_card_owner(card) {
            return Ok(false);
        }
        // First card is always valid
        // TODO: This is wrong
        if self.play_turn == 0 {
            return Ok(true);
        }
        // Can be played with the current first card?
        if self.first_card.is_some() {
            if self
                .ruleset
                .unwrap()
                .card_is_trump(&self.first_card.unwrap())
            {
                // First card is trump
                if self.ruleset.unwrap().card_is_trump(card)
                    || self.has_trump_in_hand(self.vorhand) == Ok(false)
                {
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                // First card is color
                if card.color == self.first_card.unwrap().color
                    || !self.has_color_in_hand(self.first_card.unwrap().color, self.vorhand)
                {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(true)
        }
    }

    fn has_color_in_hand(&self, color: Colors, hand: u8) -> bool {
        let mut hand_cards = self.get_cards_in_location(hand);
        for card in hand_cards.drain() {
            if card.color == color {
                return true;
            }
        }
        false
    }

    fn has_number_of_color_in_hand(&self, color: Colors, hand: u8) -> bool {
        let mut hand_cards = self.get_cards_in_location(hand);
        for card in hand_cards.drain() {
            if card.color == color
                && card.symbol != Symbols::Ober
                && card.symbol != Symbols::Unter
                && card.symbol != Symbols::Ass
            {
                return true;
            }
        }
        false
    }

    fn has_trump_in_hand(&self, hand: u8) -> Result<bool, &'static str> {
        match self.ruleset {
            None => Err("No ruleset has been chosen"),
            Some(ruleset) => {
        let mut hand_cards = self.get_cards_in_location(hand);
        for card in hand_cards.drain() {
                    if ruleset.card_is_trump(&card) {
                return Ok(true);
            }
        }
        Ok(false)
            }
        }
    }

    fn determine_round_winner(&self) -> Result<u8, &'static str> {
        if self.ruleset.is_none() {
            return Err("No ruleset has been chosen");
        }
        if self.first_card.is_none() {
            return Err("Attempted to determine winner for round that has not started");
        }
        let mut cards_in_stich = self.get_cards_in_location(0);
        let mut highest_card: Option<Card> = None;
        for (i, card) in cards_in_stich.drain().enumerate() {
            if i == 0 {
                highest_card = Some(card.clone());
                continue;
            }
            if self.ruleset.unwrap().compare_cards(
                &card,
                &highest_card.unwrap(),
                &self.first_card.unwrap(),
            ) {
                highest_card = Some(card.clone());
            }
        }
        match self.starting_hands.get(&highest_card.unwrap()) {
            Some(location) => Ok(location - 1),
            None => Err("Card has disappeared"),
        }
    }

    fn determine_game_winner(&self) -> u8 {
        let mut points: [u8; 4] = [0, 0, 0, 0];
        for card in self.deck.clone() {
            if card.1 <= 8 && card.1 >= 5 {
                points[card.1 as usize] += card.0.get_value().unwrap() as u8;
            }
        }
        points
            .iter()
            .enumerate()
            .max()
            .map(|(idx, _)| idx)
            .unwrap()
            .clone() as u8
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new_cards() {
        let starting_cards = Game::new_cards();
        assert_eq!(starting_cards.len(), 32, "Too many cards in play");
        // Count that each hand has exactly four cards and no cards elsewhere
        for location in 0..9 {
            let cards_in_location: usize =
                starting_cards.values().filter(|l| **l == location).count();
            if location >= 1 && location <= 4 {
                assert_eq!(
                    cards_in_location, 8,
                    "Player {} does not have the right card amount",
                    location
                );
            } else {
                assert_eq!(
                    cards_in_location, 0,
                    "Cards assigned outside hand at location {}",
                    location
                );
            }
        }
    }

    #[test]
    fn test_compare_cards() {
        assert!(SCHELLEN_SAUSPIEL.compare_cards(
            &Card {
                color: Colors::Gras,
                symbol: Symbols::Ober
            },
            &Card {
                color: Colors::Schelln,
                symbol: Symbols::Unter
            },
            &Card {
                color: Colors::Herz,
                symbol: Symbols::Unter
            }
        ));
        assert!(GRAS_SAUSPIEL.compare_cards(
            &Card {
                color: Colors::Eichel,
                symbol: Symbols::Zehn
            },
            &Card {
                color: Colors::Eichel,
                symbol: Symbols::Koenig
            },
            &Card {
                color: Colors::Eichel,
                symbol: Symbols::Acht
            }
        ));
        assert!(EICHEL_SAUSPIEL.compare_cards(
            &Card {
                color: Colors::Schelln,
                symbol: Symbols::Unter
            },
            &Card {
                color: Colors::Herz,
                symbol: Symbols::Ass
            },
            &Card {
                color: Colors::Schelln,
                symbol: Symbols::Ass
            }
        ));
        assert!(SCHELLEN_SAUSPIEL.compare_cards(
            &Card {
                color: Colors::Eichel,
                symbol: Symbols::Neun
            },
            &Card {
                color: Colors::Gras,
                symbol: Symbols::Zehn
            },
            &Card {
                color: Colors::Eichel,
                symbol: Symbols::Sieben
            }
        ));
    }

    #[test]
    fn test_announce_phase() {
        let mut test_game: Game = Game::new(0);
        for n in 1..5 {
            if test_game.announce_game(Some(SCHELLEN_SAUSPIEL)) {
            } else if test_game.announce_game(Some(EICHEL_SAUSPIEL)) {
            } else if test_game.announce_game(Some(GRAS_SAUSPIEL)) {
            } else {
                test_game.announce_game(None);
            }
        }
        assert_eq!(
            test_game.game_progress.state, 4,
            "Did not reach the expected game state 4. Instead it is {}.",
            test_game.game_progress.state
        );
        assert!(test_game.teams.is_some());
        let members_first_team = hamming::weight(&[test_game.teams.unwrap()]);
        assert!(members_first_team > 0 && members_first_team < 4);
    }
}
