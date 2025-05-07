use {
  crate::{card::Card, game::Game},
  std::io::{self, Write},
};

pub trait PickCard {
  fn pick_card(&mut self) -> Option<Card>;
}

impl PickCard for Game {
  fn pick_card(&mut self) -> Option<Card> {
    self.print_choices();
    let choice: Option<usize> = Self::read_choice();
    let cards: Vec<Card> = self.current_hand_cards();

    let is_valid_card: Option<usize> = choice
      .and_then(|n| n.checked_sub(1))
      .filter(|&i| i < cards.len());

    is_valid_card.map_or_else(
      || {
        println!("Invalid choice!");
        None
      },
      |index| {
        let selected: &Card = &cards[index];
        println!("You picked: {selected}");
        Some(*selected)
      },
    )
  }
}

impl Game {
  fn print_choices(&self) {
    let cards: &[Card] = self.players()[self.current_turn().turn() as usize]
      .hand()
      .cards();

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
