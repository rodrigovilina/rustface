use {
  crate::{
    back_color::BackColor, card::Card, face_down_hand::FaceDownHand, face_up_hand::FaceUpHand,
    front_color::FrontColor, hand::Hand, player::Player, rank::Rank, suit::Suit,
  },
  rand::{rng, seq::SliceRandom},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deck {
  pub cards: Vec<Card>,
}

impl Deck {
  const fn empty() -> Self {
    Self { cards: vec![] }
  }

  pub fn complete() -> Self {
    let cards: Vec<Card> = [Self::regular(), Self::jokers()].concat();

    Self { cards }
  }

  pub fn create_player(&mut self) -> Player {
    let taken: Vec<Card> = self.cards.drain(0..9).collect();
    let mut chunks = taken.chunks(3).map(|c| c.to_vec());

    let face_down: FaceDownHand = FaceDownHand::new(chunks.next().unwrap());
    let face_up: FaceUpHand = FaceUpHand::new(chunks.next().unwrap());
    let hand: Hand = Hand::new(chunks.next().unwrap());

    Player::new(face_up, face_down, hand)
  }

  pub fn shuffle(&mut self) {
    let mut rng = rng();
    self.cards.shuffle(&mut rng);
  }

  pub fn pop(&mut self) -> Option<Card> {
    self.cards.pop()
  }

  fn regular() -> Vec<Card> {
    BackColor::all()
      .into_iter()
      .flat_map(|back_color| {
        Suit::all().into_iter().flat_map(move |suit| {
          Rank::all().into_iter().map(move |number| Card::Regular{
            rank: number,
            suit,
            back_color,
          })
        })
      })
      .collect()
  }

  fn jokers() -> Vec<Card> {
    BackColor::all()
      .into_iter()
      .flat_map(|back_color| {
        FrontColor::all()
          .into_iter()
          .map(move |front_color| Card::Joker {
            back_color,
            front_color,
          })
      })
      .collect()
  }
}
