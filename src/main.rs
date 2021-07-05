use rand::Rng;
use std::fmt;
use std::io;
#[derive(Copy, Clone, PartialEq)]
enum Suite {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}
#[derive(Clone, PartialEq)]
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
        let mut count = 0;
        for i in self.cards.iter() {
            let card_str_rep = (*i).to_string();
            str_rep.push_str("(");
            str_rep.push_str(&count.to_string());
            str_rep.push_str(") ");
            str_rep.push_str(&card_str_rep);
            count += 1;
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

    fn remove_card(&mut self, to_remove: &Card) {
        self.cards.retain(|x| x != to_remove);
    }

    fn add_card(&mut self, to_add: Card) {
        self.cards.push(to_add);
    }

    fn len(&self) -> i32 {
        self.cards.len() as i32
    }

    fn deal_rest(&mut self) -> Option<Deck> {
        self.deal_hand(self.len())
    }

    fn get_cards(&self) -> Vec<Card> {
        let mut cards = Vec::<Card>::new();
        for i in &self.cards {
            cards.push(i.clone());
        }
        cards
    }
}

#[derive(Clone)]
struct Rank {
    idx: usize,
}

impl<'a> Rank {
    const RANKS: [&'static str; 13] = [
        "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
        "Queen", "King",
    ];
    fn set_ptr(&mut self, rank: &'a str) {
        self.idx = Self::RANKS.iter().position(|&r| r == rank).unwrap();
    }

    fn next(&mut self) {
        self.idx += 1;
    }
    fn peek_next(&self) -> &'static str {
        Self::RANKS[(self.idx + 1) % Self::RANKS.len()]
    }
    fn prev(&mut self) {
        self.idx -= 1;
    }
    fn peek_prev(&self) -> &'static str {
        Self::RANKS[(self.idx - 1) % Self::RANKS.len()]
    }
}
fn print_menu() {
    println!("(0) Choose pile to place card on left pile");
    println!("(1) Choose pile to place card on right pile");
    println!("(2) Wait");
    println!("(3) Exit");
}
fn get_input() -> i32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    match guess.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => -1,
    }
}
fn main() -> Result<(), &'static str> {
    let mut deck = Deck::new();
    deck.init();
    assert_eq!(deck.len(), 52);
    deck.shuffle();
    // speed starts with 2 in the middle
    let starting_two = deck.deal_hand(2).unwrap();
    let mut left_pile = Rank { idx: 0 };
    let mut right_pile = Rank { idx: 0 };
    let mut starting_two_cards = starting_two.get_cards();
    let mut right_card = starting_two_cards.pop().unwrap();
    left_pile.set_ptr(&right_card.rank);
    let mut left_card = starting_two_cards.pop().unwrap();
    right_pile.set_ptr(&left_card.rank);
    let mut player_complete_hand = deck.deal_hand(deck.len() / 2).unwrap();
    let mut computer_complete_hand = deck.deal_rest().unwrap();
    assert_eq!(deck.len(), 0);
    assert_eq!(player_complete_hand.len(), 25);
    assert_eq!(computer_complete_hand.len(), 25);
    let mut player_five = player_complete_hand.deal_hand(5).unwrap();
    let mut computer_five = computer_complete_hand.deal_hand(5).unwrap();
    loop {
        let divider = "=".repeat(16);
        println!(
            "Middle cards:\n{}\n(0){}(1){}{}\n",
            divider, left_card, right_card, divider
        );
        println!("Your cards:\n{}\n{}{}\n", divider, player_five, divider);
        print_menu();
        let guess = get_input();
        let mut left_side = true;
        if guess == 0 {
            left_side = false;
            println!("Choosing to place card on top of {}", left_card);
        } else if guess == 1 {
            println!("Choosing to place card on top of {}", right_card);
        } else if guess == 3 {
            break;
        } else if guess < 0 || guess > 2 {
            return Err("Input must be integer between 0 and 2 inclusive");
        } else if guess == 2 {
            // wait for computer to play
            // iterate through all possible playable cards for left and right
            // if you find one you can play, play -> update the correct iterator for the card played
            let computer_card_options = computer_five.get_cards();
            for guess in 0..computer_card_options.len() {
                let chosen_card = computer_card_options.get(guess as usize).clone().unwrap();
                let chosen_rank = chosen_card.rank.clone();
                if left_pile.peek_next() == chosen_rank {
                    println!("Computer placed {} on top of {}", chosen_card, right_card);
                    // update the left pile for new top card
                    right_card = Card {
                        suite: chosen_card.suite.clone(),
                        rank: chosen_rank,
                    };
                    left_pile.next();
                    // remove the card you chose
                    computer_five.remove_card(&chosen_card);
                    // and get a new one for the five
                    // computer used up all of its cards so it wins
                    if computer_complete_hand.len() == 0 {
                        println!("Computer wins!");
                        return Ok(());
                    }
                    // computer pulls a new card into its hand of 5 removing from its deck
                    match computer_complete_hand.deal_one() {
                        Some(card) => computer_five.add_card(card),
                        None => {}
                    };
                    break;
                } else if left_pile.peek_prev() == chosen_rank {
                    println!("Computer placed {} on top of {}", chosen_card, right_card);
                    // update the left pile for new top card
                    right_card = Card {
                        suite: chosen_card.suite.clone(),
                        rank: chosen_rank,
                    };
                    left_pile.prev();
                    // remove the card you chose
                    computer_five.remove_card(&chosen_card);
                    // computer used up all of its cards so it wins
                    if computer_complete_hand.len() == 0 {
                        println!("Computer wins!");
                        return Ok(());
                    }
                    // computer pulls a new card into its hand of 5 removing from its deck
                    match computer_complete_hand.deal_one() {
                        Some(card) => computer_five.add_card(card),
                        None => {}
                    }
                    break;
                } else if right_pile.peek_next() == chosen_rank {
                    println!("Computer placed {} on top of {}", chosen_card, left_card);
                    // update the right pile for new top card
                    left_card = Card {
                        suite: chosen_card.suite.clone(),
                        rank: chosen_rank,
                    };
                    right_pile.next();
                    // remove the card the player chose
                    computer_five.remove_card(&chosen_card);
                    // and get a new one for the five
                    if computer_complete_hand.len() == 0 {
                        println!("Computer wins");
                        return Ok(());
                    }
                    match computer_complete_hand.deal_one() {
                        Some(card) => computer_five.add_card(card),
                        None => {}
                    };
                    break;
                } else if right_pile.peek_prev() == chosen_rank {
                    println!("Computer placed {} on top of {}", chosen_card, left_card);
                    // update the right pile for new top card
                    left_card = Card {
                        suite: chosen_card.suite.clone(),
                        rank: chosen_rank,
                    };
                    right_pile.prev();
                    // remove the card you chose
                    computer_five.remove_card(&chosen_card);
                    // and get a new one for the five
                    if computer_complete_hand.len() == 0 {
                        println!("Computer wins!");
                        return Ok(());
                    }
                    match computer_complete_hand.deal_one() {
                        Some(card) => computer_five.add_card(card),
                        None => {}
                    };
                    break;
                }
            }
            // if both can't play -> pull from each player and computers deck
            println!(
                "Middle cards:\n{}\n(0){}(1){}{}\n",
                divider, left_card, right_card, divider
            );
        }
        println!("Choose which card");
        println!("Your cards:\n{}\n{}{}\n", divider, player_five, divider);
        let guess = get_input();
        if guess < 0 || guess > 4 {
            return Err("Input must be integer between 0 and 4 inclusive");
        }
        let player_card_options = player_five.get_cards();
        let chosen_card = player_card_options.get(guess as usize).unwrap();
        let chosen_rank = chosen_card.rank.clone();
        if left_side {
            // place card down
            if left_pile.peek_next() == chosen_rank {
                println!("You placed {} on top of {}", chosen_card, right_card);
                // update the left pile for new top card
                right_card = Card {
                    suite: chosen_card.suite.clone(),
                    rank: chosen_rank,
                };
                left_pile.next();
                // remove the card you chose
                player_five.remove_card(&chosen_card);
                // and get a new one for the five
                if player_complete_hand.len() == 0 {
                    println!("You win!");
                    break;
                }
                match player_complete_hand.deal_one() {
                    Some(card) => player_five.add_card(card),
                    None => {}
                }
            } else if left_pile.peek_prev() == chosen_rank {
                println!("You placed {} on top of {}", chosen_card, right_card);
                // update the left pile for new top card
                right_card = Card {
                    suite: chosen_card.suite.clone(),
                    rank: chosen_rank,
                };
                left_pile.prev();
                // remove the card you chose
                player_five.remove_card(&chosen_card);
                // and get a new one for the five
                if player_complete_hand.len() == 0 {
                    println!("You win!");
                    break;
                }
                match player_complete_hand.deal_one() {
                    Some(card) => player_five.add_card(card),
                    None => {}
                }
            } else {
                println!(
                    "Invalid move try again\nTried placing {} on top of {}",
                    chosen_card, right_card
                );
            }
        } else {
            // place card down
            if right_pile.peek_next() == chosen_rank {
                println!("You placed {} on top of {}", chosen_card, left_card);
                // update the right pile for new top card
                left_card = Card {
                    suite: chosen_card.suite.clone(),
                    rank: chosen_rank,
                };
                right_pile.next();
                // remove the card the player chose
                player_five.remove_card(&chosen_card);
                // and get a new one for the five
                if player_complete_hand.len() == 0 {
                    println!("You win!");
                    break;
                }
                match player_complete_hand.deal_one() {
                    Some(card) => player_five.add_card(card),
                    None => {}
                }
            } else if right_pile.peek_prev() == chosen_rank {
                println!("You placed {} on top of {}", chosen_card, left_card);
                // update the right pile for new top card
                left_card = Card {
                    suite: chosen_card.suite.clone(),
                    rank: chosen_rank,
                };
                right_pile.prev();
                // remove the card you chose
                player_five.remove_card(&chosen_card);
                // and get a new one for the five
                if player_complete_hand.len() == 0 {
                    println!("You win!");
                    break;
                }
                match player_complete_hand.deal_one() {
                    Some(card) => player_five.add_card(card),
                    None => {}
                }
            } else {
                println!(
                    "Invalid move try again\nTried placing {} on top of {}",
                    chosen_card, left_card
                );
            }
        }
    }
    Ok(())
}
