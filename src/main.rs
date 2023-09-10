mod table;
use crate::table::rulesets::*;
use crate::table::Game;

fn main() {
    let mut game = Game::new(0);
    game.set_ruleset(SAUSPIEL);
    loop {
        let hand_on_turn = game.get_cards_in_location(game.turn+1);
        for card in hand_on_turn {
            if game.play_card(&card) { break; }
        }
    }
}