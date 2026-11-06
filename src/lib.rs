pub struct Game;

impl Game {
    pub fn perform(&self, _: Operation) -> OperationResult {
        OperationResult::Success
    }
}

pub enum Operation {
    AddPlayer { name: String }
}

#[derive(PartialEq, Debug)]
pub enum OperationResult {
    Success
}
