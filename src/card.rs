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
  #[allow(dead_code)]
  const fn back_color(self) -> BackColor {
    match self {
      Self::Joker { back_color, .. } | Self::Regular { back_color, .. } => back_color,
    }
  }

  #[allow(dead_code)]
  const fn front_color(self) -> FrontColor {
    match self {
      Self::Regular { suit, .. } => suit.front_color(),
      Self::Joker { front_color, .. } => front_color,
    }
  }

  pub const fn rank(self) -> Option<Rank> {
    match self {
      Self::Regular { rank, .. } => Some(rank),
      Self::Joker { .. } => None,
    }
  }

  /// Returns `true` if the card is [`Regular`].
  ///
  /// [`Regular`]: Card::Regular
  #[must_use]
  #[allow(dead_code)]
  pub const fn is_regular(self) -> bool {
    matches!(self, Self::Regular { .. })
  }

  /// Returns `true` if the card is [`Joker`].
  ///
  /// [`Joker`]: Card::Joker
  #[must_use]
  #[allow(dead_code)]
  pub const fn is_joker(self) -> bool {
    matches!(self, Self::Joker { .. })
  }
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Regular {
        rank,
        suit,
        back_color,
      } => {
        write!(f, "{rank:?} of {suit:?} ({back_color:?})")
      }
      Self::Joker {
        back_color,
        front_color,
      } => {
        write!(f, "{front_color:?} Joker ({back_color:?})")
      }
    }
  }
}
