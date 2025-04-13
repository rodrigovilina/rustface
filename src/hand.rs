use crate::{card::Card, play_pile::PlayPile};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hand {
  cards: Vec<Card>,
}

impl Hand {
  pub fn new(cards: Vec<Card>) -> Self {
    Self { cards }
  }

  pub fn cards(&self) -> &[Card] {
    &self.cards
  }

  pub fn remove(&mut self, card: &Card) {
    if let Some(index) = self.cards.iter().position(|c| c == card) {
      self.cards.remove(index);
    }
  }

  pub fn take(&mut self, play_pile: &mut PlayPile) {
    self.cards.extend(play_pile.cards());
    play_pile.discard();
  }
}
