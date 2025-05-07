use crate::card::Card;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hand {
  cards: Vec<Card>,
}

impl Hand {
  pub const fn new(cards: Vec<Card>) -> Self {
    Self { cards }
  }

  pub fn cards(&self) -> &[Card] {
    &self.cards
  }

  pub const fn cards_mut(&mut self) -> &mut Vec<Card> {
    &mut self.cards
  }

  pub fn remove(&mut self, card: Card) {
    if let Some(index) = self.cards.iter().position(|c| *c == card) {
      self.cards.remove(index);
    }
  }

  pub fn draw(&mut self, card: Card) {
    self.cards.extend([card]);
  }

  pub fn extend(&mut self, cards: Vec<Card>) {
    self.cards.extend(cards);
  }
}
