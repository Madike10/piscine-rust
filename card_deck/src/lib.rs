use rand::Rng;
#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug)]
pub enum Rank {
    Ace, 
    King, 
    Queen,
    Jack,
    Numb(u8),
}

impl Suit {
	pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0, 4);
        Suit::translate(value)
	}

	pub fn translate(value: u8) -> Suit {
        match value {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            3 => Suit::Club,
            _ => Suit::Heart,
        }
	}
}

impl Rank {
	pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0, 13);
        Rank::translate(value)
	}

	pub fn translate(value: u8) -> Rank {
        match value {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Queen,
            3 => Rank::Jack,
            _ => Rank::Numb(value),
        }
	}
}
#[derive(Debug)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}
//
// Finally define the function winner_card which returns true if the card passed as an argument is an ace of spades.
pub fn winner_card(card: &Card) -> bool {
    match card.rank {
        Rank::Ace => true,
        _ => false,
    }
}



// And its output

// $ cargo run
// Your card is Card { suit: Club, rank: Ace }
// $
