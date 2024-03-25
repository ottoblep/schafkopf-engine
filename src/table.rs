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
    pub ruleset: Option<Ruleset>,
    pub winner: Option<u8>,
    game_progress: u8,
    round_progress: u8,
    pub vorhand: u8,
    pub first_card: Option<Card>,
    /*
    Cards in the deck can be in the following locations:
    0. In Play
    1. Hand Player1  2. Hand Player2  3. Hand Player3  4. Hand Player4
    5. Owned Player1 6.Owned Player2 7. Owned Player3 8. Owned Player4
    Game Progress can be:
    0. Choosing Ruleset 1. In Play 2. Done
    The vorhand (first player who comes out with a card) can be:
    0. Player1 1. Player2 2. Player3 3. Player4
    */
}
impl Game {
    pub fn new(dealer: u8) -> Self {
        let starting_cards = Self::new_cards();
        return Self {
            deck: starting_cards.clone(),
            starting_hands: starting_cards,
            ruleset: None,
            winner: None,
            game_progress: 0,
            round_progress: 0,
            vorhand: (dealer + 1) % 4,
            first_card: None,
        };
    }

    pub fn set_ruleset(&mut self, ruleset: Ruleset) -> bool {
        // Must be done before playing cards
        if self.ruleset.is_none() {
            self.ruleset = Some(ruleset);
            return true;
        } else {
            return false;
        }
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
        if self.is_trump(&self.first_card.unwrap()) {
            // First card is trump
            if self.is_trump(card) || !self.has_trump_in_hand(self.vorhand) {
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
            if self.is_trump(&card) {
                return true;
            }
        }
        return false;
    }

    fn is_trump(&self, card: &Card) -> bool {
        assert!(self.ruleset.is_some());
        if card.color == self.ruleset.unwrap().trump_color.unwrap()
            || card.symbol == self.ruleset.unwrap().trump_symbols[0].unwrap()
            || card.symbol == self.ruleset.unwrap().trump_symbols[1].unwrap()
        {
            return true;
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
            if self.card_is_higher(&card, &highest_card.unwrap()) {
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

    fn card_is_higher(&self, card1: &Card, card2: &Card) -> bool {
        // Trump decides
        if self.is_trump(card1) != self.is_trump(card2) {
            return self.is_trump(card1);
        }
        // First color decides
        if !self.is_trump(card1) {
            if (card1.color == self.first_card.unwrap().color)
                != (card1.color == self.first_card.unwrap().color)
            {
                return card1.color == self.first_card.unwrap().color;
            }
        }
        // Higher symbol decides
        if card1.symbol != card2.symbol {
            return card1.symbol as u8 > card2.symbol as u8;
        }
        // Higher color decides
        return card1.color as u8 > card2.color as u8;
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
}