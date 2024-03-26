pub mod cards;
pub mod rulesets;
use crate::table::cards::*;
use crate::table::rulesets::*;
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
    // Tracks the last and highest game announced
    teams: Option<u8>,
    // Only two teams can exist
    // Teams might not be known to all players at the start
    // Bits 0..3 descibe team assignment for players 1 to 4
    pub winner: Option<u8>,
    // Winner 1..4
    pub game_progress: u8,
    // Game Progress can be:
    // 0. Announcement phase 1. Play Phase 2. Done
    pub round_progress: u8,
    // Player currently defining the next action 0..3
    // also tracks the player currently able to announce a game during announcement phase
    pub vorhand: u8,
    // The vorhand (first player who comes out with a card) 0..3
    // also tracks the player that has announced the highest game during announcement phase
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
        return Self {
            deck: starting_cards.clone(),
            starting_hands: starting_cards,
            ruleset: None,
            teams: None,
            winner: None,
            game_progress: 0,
            round_progress: (dealer + 1) % 4,
            vorhand: (dealer + 1) % 4,
            first_card: None,
        };
    }

    pub fn announce_game(&mut self, announce_ruleset: Option<Ruleset>) -> Result<bool, &'static str> {
        if self.game_progress != 0 {
            return Err("Attempted to announce a game while not in choosing ruleset phase");
        }

        if self.announcement_is_valid(announce_ruleset) {
            self.vorhand = self.round_progress; // Set the new announcer 
            self.ruleset = announce_ruleset;
        } else {
            return Ok(false)
        }

        self.round_progress = self.round_progress + 1 % 4;

        // If one round is completed without a new announcement move to next phase
        if self.round_progress == self.vorhand {
            // TODO: Implement Ramsch
            if self.ruleset == None { return Err("Ramsch not implemented yet") }
            self.game_progress = 1;
            self.round_progress = 0;
        }
        return Ok(true);
    }

    pub fn announcement_is_valid(&self, announce_ruleset: Option<Ruleset>) -> bool {
        // Passing is always allowed
        if announce_ruleset.is_none() {
            return true;
        }
        // Announcement needs to be higher value than the last
        if self.ruleset.is_some() {
            if announce_ruleset.unwrap().value <= self.ruleset.unwrap().value { return false }
        }

        let announce_sow = announce_ruleset.unwrap().sow;
        if  announce_sow.is_some() {
        // Caller cannot have the sow in hand that is called
            if self.get_card_owner(&announce_sow.unwrap()) == self.round_progress { return false; }
            // Caller needs to have at least one card of the sow color which is not trump
            // TODO: check for not being trump
            if !self.has_color_in_hand(announce_sow.unwrap().color, self.round_progress) { return false; }
        }

        //

        return true;

        // TODO: complete
    }

    pub fn play_card(&mut self, card: &Card) -> Result<bool, &'static str> {
        if self.game_progress != 1 {
            return Err("Attempted to play card while not in play phase");
        }
        
        if !Self::card_is_valid(self, card) {
            return Ok(false);
        }

        // Assign first card
        if self.round_progress == 0 {
            self.first_card = Some(card.clone());
        }

        // Play card
        self.deck.insert(card.clone(), 0); // Move card into play
        self.round_progress += 1;

        // Check for end of round
        if self.round_progress == 4 {
            self.end_round();
        }

        // Check for end of game
        if self.game_progress == 4 {
            self.winner = Some(self.determine_game_winner());
        }

        return Ok(true);
    }

    pub fn get_cards_in_location(&self, location: u8) -> HashSet<Card> {
        let mut hand = HashSet::new();
        for card in self.deck.clone() {
            if card.1 == location {
                hand.insert(card.0);
            }
        }
        return hand;
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
        return deck;
    }

    fn end_round(&mut self) {
        let mut cards_in_stich = self.get_cards_in_location(0);
        let winner = self.determine_round_winner();
        for card in cards_in_stich.drain() {
            self.deck.insert(card, winner + 5); // Move cards to winner owned
        }
        self.vorhand = winner;
        self.round_progress = 0;
        self.game_progress += 1;
    }

    fn get_card_owner(&self, card: &Card) -> u8 {
        return self.starting_hands.get(card).unwrap() - 1;
    }

    fn card_is_valid(&self, card: &Card) -> bool {
        // Is in player hand whos turn it is
        if !self.vorhand == self.get_card_owner(card) {
            return false;
        }
        // First card is always valid
        if self.round_progress == 0 {
            return true;
        }
        // Can be played with the current first card?
        if self.ruleset.unwrap().card_is_trump(&self.first_card.unwrap()) {
            // First card is trump
            if self.ruleset.unwrap().card_is_trump(card) || !self.has_trump_in_hand(self.vorhand) {
                return true;
            }
        } else {
            // First card is color
            if card.color == self.first_card.unwrap().color
                || !self.has_color_in_hand(self.first_card.unwrap().color, self.vorhand)
            {
                return true;
            }
        }
        return false;
    }

    fn has_color_in_hand(&self, color: Colors, hand: u8) -> bool {
        let mut hand_cards = self.get_cards_in_location(hand);
        for card in hand_cards.drain() {
            if card.color == color {
                return true;
            }
        }
        return false;
    }

    fn has_trump_in_hand(&self, hand: u8) -> bool {
        let mut hand_cards = self.get_cards_in_location(hand);
        for card in hand_cards.drain() {
            if self.ruleset.unwrap().card_is_trump(&card) {
                return true;
            }
        }
        return false;
    }

    fn determine_round_winner(&self) -> u8 {
        let mut cards_in_stich = self.get_cards_in_location(0);
        let mut highest_card: Option<Card> = None;
        for (i, card) in cards_in_stich.drain().enumerate() {
            if i == 0 {
                highest_card = Some(card.clone());
                continue;
            }
            if self.ruleset.unwrap().compare_cards(&card, &highest_card.unwrap(), &self.first_card.unwrap()) {
                highest_card = Some(card.clone());
            }
        }
        return self.starting_hands.get(&highest_card.unwrap()).unwrap() - 1;
    }

    fn determine_game_winner(&self) -> u8 {
        let mut points: [u8; 4] = [0, 0, 0, 0];
        for card in self.deck.clone() {
            if card.1 <= 8 && card.1 >= 5 {
                points[card.1 as usize] += card.0.get_value().unwrap() as u8;
            }
        }
        return points
            .iter()
            .enumerate()
            .max()
            .map(|(idx, _)| idx)
            .unwrap()
            .clone() as u8;
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
            let cards_in_location: usize = starting_cards.values().filter(|l| **l == location).count();
            if location >= 1 && location <=4 {
                assert_eq!(cards_in_location, 8, "Player {} does not have the right card amount", location);
            } else {
                assert_eq!(cards_in_location, 0, "Cards assigned outside hand at location {}", location);
            }
        }
    }


    #[test]
    fn test_compare_cards() {
        assert!(SCHELLEN_SAUSPIEL.compare_cards(
            &Card {color: Colors::Gras, symbol: Symbols::Ober},
            &Card {color: Colors::Schelln, symbol: Symbols::Unter},
            &Card {color: Colors::Herz, symbol: Symbols::Unter}));
        assert!(GRAS_SAUSPIEL.compare_cards(
            &Card {color: Colors::Eichel, symbol: Symbols::Zehn},
            &Card {color: Colors::Eichel, symbol: Symbols::Koenig},
            &Card {color: Colors::Eichel, symbol: Symbols::Acht}));
        assert!(EICHEL_SAUSPIEL.compare_cards(
            &Card {color: Colors::Schelln, symbol: Symbols::Unter},
            &Card {color: Colors::Herz, symbol: Symbols::Ass},
            &Card {color: Colors::Schelln, symbol: Symbols::Ass}));
        assert!(SCHELLEN_SAUSPIEL.compare_cards(
            &Card {color: Colors::Eichel, symbol: Symbols::Neun},
            &Card {color: Colors::Gras, symbol: Symbols::Zehn},
            &Card {color: Colors::Eichel, symbol: Symbols::Sieben}));
        }
}