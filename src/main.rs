#![warn(clippy::complexity)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used)]

fn main() {
  println!("Hello, world!");
}

#[derive(Clone, Debug, PartialEq)]
struct PlayDeck {
  cards: Vec<Card>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
  number: Number,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Number {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  J,
  Q,
  K,
  A,
}

impl PlayDeck {
  const fn empty() -> Self {
    Self { cards: vec![] }
  }

  fn play(&mut self, card: Card) {
    self.cards.push(card);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_deck() {
    let mut play_deck = PlayDeck::empty();
    let card = Card {
      number: Number::Two,
    };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card {
      number: Number::Three,
    };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card {
      number: Number::Four,
    };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card {
      number: Number::Five,
    };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card {
      number: Number::Six,
    };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card {
      number: Number::Seven,
    };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card {
      number: Number::Eight,
    };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card {
      number: Number::Nine,
    };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card {
      number: Number::Ten,
    };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card { number: Number::J };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card { number: Number::Q };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card { number: Number::K };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);

    let mut play_deck = PlayDeck::empty();
    let card = Card { number: Number::A };
    play_deck.play(card);
    assert_eq!(play_deck.cards, vec![card]);
  }

  #[test]
  fn empty_after_two() {
    let card2 = Card {
      number: Number::Two,
    };
    let card3 = Card {
      number: Number::Three,
    };
    let card4 = Card {
      number: Number::Three,
    };

    let mut play_deck = PlayDeck::empty();
    play_deck.play(card2);
    play_deck.play(card2);
    assert_eq!(play_deck.cards, vec![card2, card2]);

    let mut play_deck = PlayDeck::empty();
    play_deck.play(card2);
    play_deck.play(card3);
    assert_eq!(play_deck.cards, vec![card2, card3]);

    let mut play_deck = PlayDeck::empty();
    play_deck.play(card2);
    play_deck.play(card4);
    assert_eq!(play_deck.cards, vec![card2, card4]);
  }
}
