use assertor::*;
use console_gang::{Game, Operation::AddPlayer};


#[test]
fn players_can_be_added() {
    let game = Game::new();

    let result = game.perform(AddPlayer { name: String::from("Alice") });

    assert_that!(result).is_equal_to(Ok(()))
}
