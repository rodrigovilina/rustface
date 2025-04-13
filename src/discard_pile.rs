use crate::{card::Card, play_pile::PlayPile};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscardPile {
  cards: Vec<Card>,
}

impl DiscardPile {
  pub fn empty() -> Self {
    Self { cards: vec![] }
  }

  pub fn discard(&mut self, play_pile: &mut PlayPile) {
    self.cards.extend(play_pile.cards());
    play_pile.discard();
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    crate::{back_color::BackColor, rank::Rank, suit::Suit},
  };

  #[test]
  fn test_discard() {
    let mut play_pile: PlayPile = PlayPile::empty();
    let mut discard_pile: DiscardPile = DiscardPile::empty();
    let card = Card::Regular {
      rank: Rank::Two,
      suit: Suit::Heart,
      back_color: BackColor::Red,
    };
    play_pile.play(card.clone());
    discard_pile.discard(&mut play_pile);

    assert_eq!(play_pile.cards(), vec![]);
    assert_eq!(discard_pile.cards, vec![card.clone()]);
  }
}
