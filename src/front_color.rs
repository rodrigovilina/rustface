#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrontColor {
  Red,
  Black,
}

impl FrontColor {
  pub const fn all() -> [Self; 2] {
    [Self::Black, Self::Red]
  }
}
