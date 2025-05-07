use crate::{game::Game, player::Player};

pub trait CurrentPlayer {
  fn current_player(&mut self) -> &mut Player;
}

impl CurrentPlayer for Game {
  fn current_player(&mut self) -> &mut Player {
    let current_turn = self.current_turn().turn() as usize;
    &mut self.players_mut()[current_turn]
  }
}

pub trait NextPlayer {
  fn next_player(&mut self) -> &mut Player;
}

impl NextPlayer for Game {
  fn next_player(&mut self) -> &mut Player {
    let current_turn = self.current_turn();
    let next_turn = current_turn.next_in_dir() as usize;
    &mut self.players_mut()[next_turn]
  }
}
