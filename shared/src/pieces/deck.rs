use rand::prelude::*;

pub struct Deck<C: std::clone::Clone> {
  pub cards: Vec<C>,
}

impl<C: std::clone::Clone> Deck<C> {
  pub fn new(cards: &[C]) -> Deck<C> {
    Deck {
      cards: Vec::from(cards),
    }
  }

  pub fn shuffle(&mut self) {
    let mut rng = rand::thread_rng();

    self.cards.shuffle(&mut rng);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn shuffle() {
    let mut deck = Deck {
      cards: vec![1, 2, 3, 4],
    };

    deck.shuffle();
  }
}
