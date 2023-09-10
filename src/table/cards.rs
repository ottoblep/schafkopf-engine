use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Colors {
    Eichel,
    Gras,
    Herz,
    Schelln,
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash)]
pub enum Symbols {
    Ass,
    Zehn,
    Koenig,
    Ober,
    Unter,
    Neun,
    Acht,
    Sieben,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub symbol: Symbols,
    pub color: Colors,
}
impl Card {
    pub fn get_value(self) -> i16 {
        match self.symbol {
            Symbols::Ass => return 11,
            Symbols::Zehn => return 10,
            Symbols::Koenig => return 4,
            Symbols::Ober => return 3,
            Symbols::Unter => return 2,
            _ => return 0,
        }
    }
}
