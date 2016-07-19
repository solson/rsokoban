extern crate rsokoban;

use rsokoban::{GameState, Action};

fn main() {
    let mut game = GameState::new();

    loop {
        match game.handle_input() {
            Action::Stop => break,
            Action::None => {},
        }

        game.update();
        game.render();
    }
}
