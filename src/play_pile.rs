use crate::{card::Card, rank::Rank};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayPile {
  cards: Vec<Card>,
}

impl PlayPile {
  pub fn empty() -> Self {
    Self { cards: vec![] }
  }

  pub fn discard(&mut self) {
    self.cards = vec![];
  }

  pub fn cards(&self) -> Vec<Card> {
    self.cards.clone()
  }

  pub fn top_card(&self) -> Option<&Card> {
    self.cards.first()
  }

  fn top_four_same_rank(&self) -> bool {
    if self.cards.len() < 4 {
      return false;
    }

    let first_rank = &self.cards[0].rank();
    self.cards[1..4].iter().all(|card| &card.rank() == first_rank)
  }

  pub fn play(&mut self, card: Card) {
    self.cards.insert(0, card);
  }

  pub fn can_play_any(&self, cards: &[Card]) -> bool {
    cards.iter().any(|card| self.can_play_card(card))
  }

  pub fn can_play_card(&self, card: &Card) -> bool {
    if let Some(top_card) = self.top_card() {
      Self::can_play_against(top_card, card)
    } else {
      true
    }
  }

  fn can_play_against(top: &Card, card: &Card) -> bool {
    match (top, card) {
      (Card::Regular { rank: Rank::Ten, .. }, _) => false,
      (_, Card::Joker { .. }) => true,
      (Card::Joker { .. }, Card::Regular { .. }) => false,
      (
        Card::Regular { rank: top_rank, .. },
        Card::Regular { rank: card_rank, .. },
      ) => Self::can_play_rank(top_rank, card_rank),
    }
  }

  fn can_play_rank(top: &Rank, card: &Rank) -> bool {
    match (top, card) {
      (Rank::Ten, _) => false,
      (Rank::Seven, b) => *b <= Rank::Seven,
      (_, Rank::Two) => true,
      (a, b) => a <= b,
    }
  }
}
