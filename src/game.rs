use {
  crate::{
    card::Card, deck::Deck, discard_pile::DiscardPile, hand::Hand, play_pile::PlayPile,
    player::Player,
  },
  std::io::{self, Write},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
  Left,
  #[allow(dead_code)]
  Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
  current_turn: u8,
  deck: Deck,
  direction: Direction,
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
    let direction: Direction = Direction::Left;
    Self {
      current_turn: 0,
      deck,
      direction,
      discard_pile,
      play_pile,
      players,
    }
  }

  pub fn take_turn(&mut self) {
    self.print_choices();
    let player: &mut Player = &mut self.players[self.current_turn as usize];
    let hand: &mut Hand = player.hand_mut();
    let cards: Vec<Card> = hand.cards().to_vec();

    if self.play_pile.can_play_any(&cards) {
      let choice: Option<usize> = Self::read_choice();

      if let Some(index) = choice
        .and_then(|n| n.checked_sub(1))
        .filter(|&i| i < cards.len())
      {
        let selected: &Card = &cards[index];
        println!("You picked: {selected}");

        if self.play_pile.can_play_card(*selected) {
          hand.remove(*selected);
          if let Some(card) = self.deck.pop() {
            hand.draw(card);
          }
          self.play_pile.play(*selected);
          // pick hand from deck
          println!("you played the card");
          dbg!(&player.hand());
          dbg!(&self.play_pile);
          self.increase_turn();
        } else {
          println!("you cannot play the card");
        }
      } else {
        println!("Invalid choice!");
      }
    } else {
      println!("You cannot play any of your cards");
      hand.take(&mut self.play_pile);
      self.decrease_turn();
    }
  }

  const fn increase_turn(&mut self) {
    self.current_turn += 1;
    self.current_turn %= self.players.len() as u8;
  }

  const fn decrease_turn(&mut self) {
    match self.current_turn {
      0 => self.current_turn = self.players.len() as u8 - 1,
      _ => self.current_turn -= 1,
    }
  }

  fn print_choices(&self) {
    let player: &Player = &self.players[self.current_turn as usize];
    let hand: &Hand = player.hand();
    let cards: &[Card] = hand.cards();

    println!("Pick a card:");
    for (i, card) in cards.iter().enumerate() {
      println!("{}: {}", i + 1, card);
    }
  }

  fn read_choice() -> Option<usize> {
    print!("Enter the number of the card you want: ");
    io::stdout().flush().unwrap(); // Ensure prompt shows before input

    let mut input: String = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let choice: Option<usize> = input.trim().parse::<usize>().ok();
    choice
  }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
