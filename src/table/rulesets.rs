use crate::table::cards::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ruleset {
    pub trump_color: Option<Colors>,
    pub trump_symbols: [Option<Symbols>; 2],
    pub sow: Option<Card>,
    pub value: u8,
}
impl Ruleset {
    pub fn card_is_trump(&self, card: &Card) -> bool {
        return self.trump_color == Some(card.color)
            || self.trump_symbols[0] == Some(card.symbol)
            || self.trump_symbols[1] == Some(card.symbol);
    }

    pub fn compare_cards(&self, card1: &Card, card2: &Card, first_card: &Card) -> bool {
        // Trump decides
        if self.card_is_trump(card1) != self.card_is_trump(card2) {
            return self.card_is_trump(card1);
        }
        // First color decides
        if !self.card_is_trump(card1) {
            if (card1.color == first_card.color) != (card2.color == first_card.color) {
                return card1.color == first_card.color;
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

pub static EICHEL_SAUSPIEL: Ruleset = Ruleset {
    trump_color: Some(Colors::Herz),
    trump_symbols: [Some(Symbols::Unter), Some(Symbols::Ober)],
    sow: Some(Card {
        color: Colors::Eichel,
        symbol: Symbols::Ass,
    }),
    value: 1,
};

pub static GRAS_SAUSPIEL: Ruleset = Ruleset {
    trump_color: Some(Colors::Herz),
    trump_symbols: [Some(Symbols::Unter), Some(Symbols::Ober)],
    sow: Some(Card {
        color: Colors::Gras,
        symbol: Symbols::Ass,
    }),
    value: 1,
};

pub static SCHELLEN_SAUSPIEL: Ruleset = Ruleset {
    trump_color: Some(Colors::Herz),
    trump_symbols: [Some(Symbols::Unter), Some(Symbols::Ober)],
    sow: Some(Card {
        color: Colors::Schelln,
        symbol: Symbols::Ass,
    }),
    value: 1,
};

pub static WENZ: Ruleset = Ruleset {
    trump_color: None,
    trump_symbols: [Some(Symbols::Unter), None],
    sow: None,
    value: 2,
};
