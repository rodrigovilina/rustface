#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackColor {
  Blue,
  Red,
}

impl BackColor {
  pub const fn all() -> [Self; 2] {
    [Self::Blue, Self::Red]
  }
}
