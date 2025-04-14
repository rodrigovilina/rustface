use crate::card::Card;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaceUpHand {
  cards: Vec<Card>,
}

impl FaceUpHand {
  pub const fn new(cards: Vec<Card>) -> Self {
    Self { cards }
  }
}
