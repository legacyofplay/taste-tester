use shared::*;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

fn main() {
    let mut deck = Deck::new(&[Card {
        suit: Suit::Spades,
        value: 1,
    }]);

    deck.shuffle();

    println!("Top: {:?}", deck.cards.first());
}
