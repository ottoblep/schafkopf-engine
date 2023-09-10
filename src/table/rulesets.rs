use crate::table::cards::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ruleset {
    pub trump_color: Option<Colors>,
    pub trump_symbols: [Option<Symbols>; 2],
    pub has_teams: bool,
}
impl Ruleset {
    fn card_is_trump(&self, card: &Card) -> bool {
        return self.trump_color == Some(card.color)
            || self.trump_symbols[0] == Some(card.symbol)
            || self.trump_symbols[1] == Some(card.symbol);
    }
}

pub static SAUSPIEL: Ruleset = Ruleset {
    trump_color: Some(Colors::Herz),
    trump_symbols: [Some(Symbols::Unter), Some(Symbols::Ober)],
    has_teams: true,
};

pub static WENZ: Ruleset = Ruleset {
    trump_color: None,
    trump_symbols: [Some(Symbols::Unter), None],
    has_teams: true,
};
