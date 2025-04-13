use {
  crate::{back_color::BackColor, front_color::FrontColor, rank::Rank, suit::Suit},
  std::fmt,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Card {
  Regular {
    rank: Rank,
    suit: Suit,
    back_color: BackColor,
  },
  Joker {
    back_color: BackColor,
    front_color: FrontColor,
  },
}

impl Card {
  const fn back_color(&self) -> BackColor {
    match self {
      Self::Joker { back_color, .. } | Self::Regular { back_color, .. } => *back_color,
    }
  }

  const fn front_color(&self) -> FrontColor {
    match self {
      Self::Regular { suit, .. } => suit.front_color(),
      Self::Joker { front_color, .. } => *front_color,
    }
  }

  pub fn rank(&self) -> Option<Rank> {
    match self {
      Card::Regular { rank, .. } => Some(rank.clone()),
      Card::Joker { .. } => None,
    }
  }

  /// Returns `true` if the card is [`Regular`].
  ///
  /// [`Regular`]: Card::Regular
  #[must_use]
  pub fn is_regular(&self) -> bool {
    matches!(self, Self::Regular { .. })
  }

  /// Returns `true` if the card is [`Joker`].
  ///
  /// [`Joker`]: Card::Joker
  #[must_use]
  pub fn is_joker(&self) -> bool {
    matches!(self, Self::Joker { .. })
  }
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Card::Regular {
        rank,
        suit,
        back_color,
      } => {
        write!(f, "{:?} of {:?} ({:?})", rank, suit, back_color)
      }
      Card::Joker {
        back_color,
        front_color,
      } => {
        write!(f, "{:?} Joker ({:?})", front_color, back_color)
      }
    }
  }
}
