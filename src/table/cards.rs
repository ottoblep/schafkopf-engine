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
    pub fn get_value(self) -> Result<i16, &'static str> {
        match self.symbol {
            Symbols::Ass => return Ok(11),
            Symbols::Zehn => return Ok(10),
            Symbols::Koenig => return Ok(4),
            Symbols::Ober => return Ok(3),
            Symbols::Unter => return Ok(2),
            _ => return Ok(0),
        }
    }
}
