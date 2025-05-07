use crate::front_color::FrontColor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suit {
  Club,
  Diamond,
  Heart,
  Spade,
}

impl Suit {
  pub const fn all() -> [Self; 4] {
    [Self::Club, Self::Diamond, Self::Heart, Self::Spade]
  }

  pub const fn front_color(self) -> FrontColor {
    match self {
      Self::Diamond | Self::Heart => FrontColor::Red,
      Self::Club | Self::Spade => FrontColor::Black,
    }
  }
}
