use assertor::*;
use console_gang::{Game, Operation::AddPlayer, OperationResult::Success};

#[test]
fn players_can_be_added() {
    let result =
        Game.perform(AddPlayer { name: String::from("Alice") });

    assert_that!(result).is_equal_to(Success)
}
