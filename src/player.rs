use crate::{face_down_hand::FaceDownHand, face_up_hand::FaceUpHand, hand::Hand};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
  face_up: FaceUpHand,
  face_down: FaceDownHand,
  hand: Hand,
}

impl Player {
  pub fn new(face_up: FaceUpHand, face_down: FaceDownHand, hand: Hand) -> Self {
    Self {
      face_up,
      face_down,
      hand,
    }
  }

  pub fn hand(&self) -> &Hand {
    &self.hand
  }

  pub fn hand_mut(&mut self) -> &mut Hand {
    &mut self.hand
  }
}
