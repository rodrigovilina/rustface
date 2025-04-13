#![warn(clippy::complexity)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used)]

mod back_color;
mod card;
mod deck;
mod discard_pile;
mod face_down_hand;
mod face_up_hand;
mod front_color;
mod game;
mod hand;
mod play_pile;
mod player;
mod rank;
mod suit;

use game::Game;

fn main() {
  let mut game: Game = Game::new();

  loop {
    game.take_turn();
  }
}
