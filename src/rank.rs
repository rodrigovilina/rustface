#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Rank {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  J,
  Q,
  K,
  A,
}

impl Rank {
  pub const fn all() -> [Self; 13] {
    [
      Self::A,
      Self::Two,
      Self::Three,
      Self::Four,
      Self::Five,
      Self::Six,
      Self::Seven,
      Self::Eight,
      Self::Nine,
      Self::Ten,
      Self::J,
      Self::Q,
      Self::K,
    ]
  }
}
