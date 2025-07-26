use crate::{
  card::Card,
  current_hand::CurrentHand,
  current_player::NextPlayer,
  current_turn::{CurrentTurn, Turn},
  deck::Deck,
  direction::{Direction, InvertDirection},
  discard_pile::DiscardPile,
  hand::Hand,
  pick_card::PickCard,
  play_pile::PlayPile,
  player::Player,
  take_shit::TakeShit,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
  current_turn: CurrentTurn,
  deck: Deck,
  discard_pile: DiscardPile,
  play_pile: PlayPile,
  players: Vec<Player>,
}

impl Game {
  pub fn new() -> Self {
    let mut deck: Deck = Deck::complete();
    deck.shuffle();
    let players = vec![deck.create_player(), deck.create_player()];
    let discard_pile: DiscardPile = DiscardPile::empty();
    let play_pile: PlayPile = PlayPile::empty();
    let current_turn: CurrentTurn = CurrentTurn::new(players.len() as u8);
    Self {
      current_turn,
      deck,
      discard_pile,
      play_pile,
      players,
    }
  }

  pub const fn current_turn(&self) -> CurrentTurn {
    self.current_turn
  }

  pub const fn current_turn_mut(&mut self) -> &mut CurrentTurn {
    &mut self.current_turn
  }

  pub const fn direction(&self) -> Direction {
    self.current_turn().direction()
  }

  pub const fn play_pile(&self) -> &PlayPile {
    &self.play_pile
  }

  pub const fn play_pile_mut(&mut self) -> &mut PlayPile {
    &mut self.play_pile
  }

  pub const fn players(&self) -> &Vec<Player> {
    &self.players
  }

  pub const fn players_mut(&mut self) -> &mut Vec<Player> {
    &mut self.players
  }

  pub fn take_turn(&mut self) {
    if self.can_play_any() {
      self.play_turn();
    } else {
      self.take_shit();
    }
  }

  fn play(&mut self, selected: Card) {
    println!("Adding the following card to the play pile: {selected}");
    self.play_pile.play(selected);
  }

  pub fn current_hand_cards(&mut self) -> Vec<Card> {
    self.current_hand().cards().to_vec()
  }

  fn can_play_any(&mut self) -> bool {
    let cards: Vec<Card> = self.current_hand_cards();
    self.play_pile.can_play_any(&cards)
  }

  fn invert_if_seven(&mut self, card: Card) {
    if card.is_seven() {
      self.invert_direction();
    }
  }

  fn discard_if_ten(&mut self) -> bool {
    self
      .play_pile
      .top_card()
      .map(|card| card.is_ten())
      .filter(|b| *b)
      .inspect(|_| self.discard_play_pile())
      .unwrap_or(false)
  }

  fn give_shit_if_joker(&mut self) -> bool {
    self
      .play_pile
      .top_card()
      .map(|card| card.is_joker())
      .filter(|b| *b)
      .inspect(|_| {
        let cards = self.play_pile.cards();
        self.next_player().take_shit(cards);
        self.play_pile.discard();
      })
      .unwrap_or(false)
  }

  fn burn_shit(&mut self) -> bool {
    if self.play_pile.top_four_same_rank() {
      self.discard_play_pile();
      true
    } else {
      false
    }
  }

  fn discard_play_pile(&mut self) {
    self.discard_pile.discard(self.play_pile.cards());
    self.play_pile.discard();
  }

  fn play_turn(&mut self) {
    if let Some(selected) = self.pick_card() {
      if self.play_pile.can_play_card(selected) {
        let turn = self.current_turn().turn() as usize;
        let player: &mut Player = &mut self.players[turn];
        let hand: &mut Hand = player.hand_mut();
        hand.remove(selected);
        if hand.cards().len() < 3
          && let Some(card) = self.deck.pop()
        {
          hand.draw(card);
        }

        self.play(selected);
        self.invert_if_seven(selected);
        let played_ten: bool = self.discard_if_ten();
        let played_joker: bool = self.give_shit_if_joker();
        let shit_burnt: bool = self.burn_shit();

        println!("you played the card");
        dbg!(self.direction());
        dbg!(&self.play_pile);
        if !played_ten && !played_joker && !shit_burnt {
          self.increase_turn();
        }
      } else {
        println!("you cannot play the card");
      }
    }
  }
}

impl Default for Game {
  fn default() -> Self {
    Self::new()
  }
}
