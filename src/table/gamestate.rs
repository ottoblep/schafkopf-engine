#[derive(PartialEq)]
pub enum GameState {
    AnnouncementPhase,
    PlayPhase,
    Done { winner: u8 }
}
impl GameState {
    pub fn advance(&mut self) -> Result<(), &'static str> {
        if *self == Self::AnnouncementPhase {
            *self == Self::PlayPhase;
            Ok(())
        } else {
            Err("Cannot advance game")
        }
    }
    pub fn finish(&mut self, winner: u8) -> Result<(), &'static str> {
        if winner <= 3 && winner >= 0 && *self == Self::PlayPhase {
            *self = Self::Done { winner: winner }; // chicken dinner
            Ok(()) 
        } else {
            Err("Cant finish game")
        }
    }
}