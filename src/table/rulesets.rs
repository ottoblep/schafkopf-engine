use crate::table::cards::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ruleset {
    pub trump_color: Option<Colors>,
    pub trump_symbols: [Option<Symbols>; 2],
    pub sow: Option<Card>,
    pub value: u8
}
impl Ruleset {
    fn card_is_trump(&self, card: &Card) -> bool {
        return self.trump_color == Some(card.color)
            || self.trump_symbols[0] == Some(card.symbol)
            || self.trump_symbols[1] == Some(card.symbol);
    }
}

pub static EICHEL_SAUSPIEL: Ruleset = Ruleset {
    trump_color: Some(Colors::Herz),
    trump_symbols: [Some(Symbols::Unter), Some(Symbols::Ober)],
    sow: Some(Card { color: Colors::Eichel, symbol: Symbols::Ass }),
    value: 0
};

pub static GRAS_SAUSPIEL: Ruleset = Ruleset {
    trump_color: Some(Colors::Herz),
    trump_symbols: [Some(Symbols::Unter), Some(Symbols::Ober)],
    sow: Some(Card { color: Colors::Gras, symbol: Symbols::Ass }),
    value: 0
};

pub static SCHELLEN_SAUSPIEL: Ruleset = Ruleset {
    trump_color: Some(Colors::Herz),
    trump_symbols: [Some(Symbols::Unter), Some(Symbols::Ober)],
    sow: Some(Card { color: Colors::Schelln, symbol: Symbols::Ass }),
    value: 0
};

pub static WENZ: Ruleset = Ruleset {
    trump_color: None,
    trump_symbols: [Some(Symbols::Unter), None],
    sow: None,
    value: 1
};
