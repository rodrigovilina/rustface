use crate::card::Card;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaceDownHand {
  cards: Vec<Card>,
}

impl FaceDownHand {
  pub fn new(cards: Vec<Card>) -> Self {
    Self { cards }
  }
}
