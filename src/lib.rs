pub struct Game;

impl Game {
    pub fn new() -> Game {
        Game
    }

    pub fn perform(&self, _: Operation) -> Result<(), ()> {
        Ok(())
    }
}

pub enum Operation {
    AddPlayer { name: String }
}
