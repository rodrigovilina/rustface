use crate::card::Card;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscardPile {
  cards: Vec<Card>,
}

impl DiscardPile {
  pub const fn empty() -> Self {
    Self { cards: vec![] }
  }

  pub fn discard(&mut self, cards: Vec<Card>) {
    self.cards.extend(cards);
  }
}
