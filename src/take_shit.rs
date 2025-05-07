use crate::{card::Card, current_hand::CurrentHand, current_turn::Turn, game::Game, hand::Hand};

pub trait TakeShit {
  fn take_shit(&mut self);
}

impl TakeShit for Game {
  fn take_shit(&mut self) {
    println!("You cannot play any of your cards");

    let cards: Vec<Card> = self.play_pile_cards();
    let hand: &mut Hand = self.current_hand();
    hand.extend(cards);
    self.play_pile_mut().discard();
    self.decrease_turn();
  }
}

impl Game {
  fn play_pile_cards(&self) -> Vec<Card> {
    self.play_pile().cards()
  }
}
