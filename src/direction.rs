use crate::{current_turn::CurrentTurn, game::Game};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
  Left,
  Right,
}

impl Direction {
  pub const fn invert(&mut self) {
    *self = match self {
      Self::Left => Self::Right,
      Self::Right => Self::Left,
    }
  }
}

pub trait InvertDirection {
  fn invert_direction(&mut self);
}

impl InvertDirection for CurrentTurn {
  fn invert_direction(&mut self) {
    self.direction_mut().invert();
  }
}

impl InvertDirection for Game {
  fn invert_direction(&mut self) {
    self.current_turn_mut().invert_direction();
  }
}
