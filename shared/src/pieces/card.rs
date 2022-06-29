#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Card<S> {
  pub suit: S,
  pub value: i8,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, PartialEq)]
  enum Suit {
    Hearts,
    Spades,
  }

  #[test]
  fn equality() {
    assert_eq!(
      Card {
        suit: Suit::Hearts,
        value: 3
      },
      Card {
        suit: Suit::Hearts,
        value: 3
      }
    );
  }

  #[test]
  fn value_inequality() {
    assert_ne!(
      Card {
        suit: Suit::Hearts,
        value: 3
      },
      Card {
        suit: Suit::Hearts,
        value: 30
      }
    );
  }

  #[test]
  fn suit_inequality() {
    assert_ne!(
      Card {
        suit: Suit::Hearts,
        value: 3
      },
      Card {
        suit: Suit::Spades,
        value: 3
      }
    );
  }

  // Does not compile (by design).
  // Could test with external help:
  // - https://github.com/rust-lang/rust/issues/12335
  // - https://github.com/Manishearth/compiletest-rs
  //
  // #[test]
  // fn type_inequality() {
  //     #[derive(Debug, PartialEq)]
  //     enum WrongSuit {
  //         Clubs,
  //     }

  //     assert_ne!(
  //         Card {
  //             suit: Suit::Hearts,
  //             value: 3
  //         },
  //         Card {
  //             suit: WrongSuit::Clubs,
  //             value: 3
  //         }
  //     );
  // }
}
