
pub struct GameState {
    pub state: u8,
    // 0. Start
    // 1. Announce1 2. Announce2 3. Announce3
    // 4. Play1 5. Play2 6. Play3 7. Play4
    // 8. Done
}
impl GameState {
    pub fn new() -> Self {
        Self { state: 0 }
    }
    pub fn AnnounceNone(&mut self) {
        match self.state {
            0 => { self.state = 1 },
            1 => { self.state = 2 },
            2 => { self.state = 3 },
            3 => { self.state = 4 },
            _ => ()
        }
    }
    pub fn AnnounceSome(&mut self) {
        match self.state {
            0 => { self.state = 1 },
            2 => { self.state = 1 },
            3 => { self.state = 1 },
            _ => ()
        }
    }
    pub fn PlayCard(&mut self) {
        match self.state {
            4 => { self.state = 5 },
            5 => { self.state = 6 },
            6 => { self.state = 7 },
            _ => ()
        }
    }
    pub fn NextRound(&mut self) {
        match self.state {
            7 => { self.state = 4 },
            _ => ()
        }
    }
    pub fn Finish(&mut self) {
        match self.state {
            7 => { self.state = 8 },
            _ => ()
        }
    }
}