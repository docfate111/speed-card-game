use rand::Rng;
use std::fmt;
use std::io::Error;
enum Suite {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}
pub struct Card {
    suite: Suite,
    rank: String,
}

impl fmt::Display for Card {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str_rep = self.to_string();
        fmt.write_str(&str_rep)?;
        Ok(())
    }
}

impl Card {
    fn new(suite: Suite, rank: String) -> Card {
        Card {
            suite: suite,
            rank: rank,
        }
    }

    fn to_string(&self) -> String {
        match self.suite {
            Suite::Clubs => format!("{} of Clubs\n", self.rank),
            Suite::Spades => format!("{} of Spades\n", self.rank),
            Suite::Diamonds => format!("{} of Diamonds\n", self.rank),
            Suite::Hearts => format!("{} of Hearts\n", self.rank),
        }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl fmt::Display for Deck {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut str_rep = "".to_owned();
        for i in self.cards.iter() {
            let card_str_rep = (*i).to_string();
            str_rep.push_str(&card_str_rep);
        }
        fmt.write_str(&str_rep)?;
        Ok(())
    }
}

impl Deck {
    fn new() -> Deck {
        let cards = Vec::<Card>::new();
        Deck { cards: cards }
    }

    fn init(&mut self) {
        let ranks = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];
        if self.cards.len() != 0 {
            return;
        }
        for rank in ranks {
            self.cards
                .push(Card::new(Suite::Diamonds, String::from(rank)));
            self.cards
                .push(Card::new(Suite::Spades, String::from(rank)));
            self.cards.push(Card::new(Suite::Clubs, String::from(rank)));
            self.cards
                .push(Card::new(Suite::Hearts, String::from(rank)));
        }
    }

    fn shuffle(&mut self) {
        for _ in 0..(self.cards.len() * 3) {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..self.cards.len());
            let index2 = rng.gen_range(0..self.cards.len());
            if index != index2 {
                self.cards.swap(index, index2);
            }
        }
    }

    fn deal_one(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn deal_hand(&mut self, num_of_cards: i32) -> Option<Deck> {
        if num_of_cards > self.cards.len() as i32 {
            return None;
        }
        let mut hand = Vec::<Card>::new();
        for _ in 0..num_of_cards {
            hand.push(self.cards.pop().unwrap());
        }
        Some(Deck { cards: hand })
    }

    fn len(&self) -> i32 {
        self.cards.len() as i32
    }

    fn deal_rest(&mut self) -> Option<Deck> {
        self.deal_hand(self.len())
    }
}

fn main() -> Result<(), Error> {
    let mut deck = Deck::new();
    deck.init();
    assert_eq!(deck.len(), 52);
    deck.shuffle();
    // speed starts with 2 in the middle
    let starting_two = deck.deal_hand(2).unwrap();
    let player_complete_hand = deck.deal_hand(deck.len() / 2).unwrap();
    let computer_complete_hand = deck.deal_rest().unwrap();
    assert_eq!(deck.len(), 0);
    assert_eq!(player_complete_hand.len(), 25);
    assert_eq!(computer_complete_hand.len(), 25);
    println!("Middle cards:\n{}", starting_two);
    Ok(())
}
