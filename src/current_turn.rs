use crate::{direction::Direction, game::Game};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CurrentTurn {
  direction: Direction,
  players: u8,
  turn: u8,
}

impl CurrentTurn {
  pub const fn new(players: u8) -> Self {
    Self {
      direction: Direction::Left,
      players,
      turn: 0,
    }
  }

  pub const fn direction(self) -> Direction {
    self.direction
  }

  pub const fn direction_mut(&mut self) -> &mut Direction {
    &mut self.direction
  }

  pub const fn turn(self) -> u8 {
    self.turn
  }

  const fn increase(&mut self) {
    self.turn = self.next_in_dir();
  }

  const fn decrease(&mut self) {
    self.turn = self.prev_in_dir();
  }

  pub const fn next_in_dir(self) -> u8 {
    match self.direction {
      Direction::Left => self.next(),
      Direction::Right => self.prev(),
    }
  }

  pub const fn prev_in_dir(self) -> u8 {
    match self.direction {
      Direction::Left => self.prev(),
      Direction::Right => self.next(),
    }
  }

  const fn next(self) -> u8 {
    (self.turn + 1) % self.players
  }

  const fn prev(self) -> u8 {
    match self.turn {
      0 => self.players - 1,
      _ => self.turn - 1,
    }
  }
}

pub trait Turn {
  fn increase_turn(&mut self);
  fn decrease_turn(&mut self);
}

impl Turn for Game {
  fn increase_turn(&mut self) {
    self.current_turn_mut().increase();
  }

  fn decrease_turn(&mut self) {
    self.current_turn_mut().decrease();
  }
}
