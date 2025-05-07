use crate::{current_player::CurrentPlayer, game::Game, hand::Hand};

pub trait CurrentHand {
  fn current_hand(&mut self) -> &mut Hand;
}

impl CurrentHand for Game {
  fn current_hand(&mut self) -> &mut Hand {
    self.current_player().hand_mut()
  }
}
