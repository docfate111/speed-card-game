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
#[derive(Copy, Clone, PartialEq)]
pub struct Card {
    suite: Suite,
    rank: &'static str,
}

const RANKS: &'static [&str] = &[
    "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen",
    "King",
];
impl fmt::Display for Card {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str_rep = self.to_string();
        fmt.write_str(&str_rep)?;
        Ok(())
    }
}

impl Card {
    fn new(suite: Suite, rank: &'static str) -> Card {
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

#[derive(Clone, PartialEq)]
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
        if self.cards.len() != 0 {
            return;
        }
        for rank in RANKS {
            self.cards.push(Card::new(Suite::Diamonds, rank));
            self.cards.push(Card::new(Suite::Spades, rank));
            self.cards.push(Card::new(Suite::Clubs, rank));
            self.cards.push(Card::new(Suite::Hearts, rank));
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
struct Pile {
    idx: usize,
    top: Card,
}

impl<'a> Pile {
    fn new(card: &Card) -> Pile {
        Pile {
            idx: RANKS.iter().position(|&r| r == card.rank).unwrap(),
            top: *card,
        }
    }
    // put a card on top of the current one
    // return None if card has been placed
    fn place_card(&mut self, card: Card) -> Option<Card> {
        let rank = card.rank;
        if RANKS[(((self.idx as isize) + 1).rem_euclid(RANKS.len() as isize) as usize)] == rank {
            self.top = card;
            println!("Placed {} on top of {}", card, RANKS[self.idx]);
            self.idx = ((self.idx as isize) + 1).rem_euclid(RANKS.len() as isize) as usize;
            None
        } else if RANKS[(((self.idx as isize) - 1).rem_euclid(RANKS.len() as isize) as usize)]
            == rank
        {
            self.top = card;
            println!("Placed {} on top of {}", card, RANKS[self.idx]);
            self.idx = ((self.idx as isize) + 1).rem_euclid(RANKS.len() as isize) as usize;
            None
        } else {
            Some(card)
        }
    }

    fn get_top(&self) -> Card {
        self.top
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
fn get_player_card_choice(divider: &String, player_five: &Deck) -> Option<i32> {
    println!("Choose which card");
    println!("Your cards:\n{}\n{}{}\n", divider, player_five, divider);
    let guess = get_input();
    if guess < 0 || guess > 4 {
        None
    } else {
        Some(guess)
    }
}

fn play_card(left_pile: &mut Pile, right_pile: &mut Pile, chosen_card: Card) -> Option<Card> {
    match left_pile.place_card(chosen_card) {
        None => None,
        Some(card) => match right_pile.place_card(card) {
            None => None,
            Some(card) => Some(card),
        },
    }
}

fn play_card_pile(pile: &mut Pile, chosen_card: Card) -> Option<Card> {
    match pile.place_card(chosen_card) {
        None => None,
        Some(card) => Some(card),
    }
}
fn main() -> Result<(), &'static str> {
    let mut deck = Deck::new();
    deck.init();
    deck.shuffle();
    // speed starts with 2 in the middle
    let starting_two = deck.deal_hand(2).unwrap();
    let mut starting_two_cards = starting_two.get_cards();
    let mut left_card = starting_two_cards.pop().unwrap();
    let mut left_pile = Pile::new(&left_card);
    let mut right_card = starting_two_cards.pop().unwrap();
    let mut right_pile = Pile::new(&right_card);
    let mut player_complete_hand = deck.deal_hand(deck.len() / 2).unwrap();
    let mut computer_complete_hand = deck.deal_rest().unwrap();
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
        if guess == 0 || guess == 1 {
            let card = if guess == 0 { left_card } else { right_card };
            println!("Choosing to place card on top of {}", card);
            let index =
                get_player_card_choice(&divider, &player_five).expect("Error: Invalid choice");
            let player_cards = player_five.get_cards();
            let chosen_card = player_cards.get(index as usize).unwrap();
            let mut pile = if guess == 0 {
                left_pile.clone()
            } else {
                right_pile.clone()
            };
            match play_card_pile(&mut pile, *chosen_card) {
                None => {
                    player_five.remove_card(&chosen_card);
                    match player_complete_hand.deal_one() {
                        None => {}
                        Some(new_card) => player_five.add_card(new_card),
                    }
                    if player_five.len() == 0 {
                        println!("Computer wins! It is out of cards!");
                        return Ok(());
                    }
                }
                Some(_) => {
                    println!("You played an invalid move!");
                }
            }
            if guess == 0 {
                left_pile = pile;
            } else {
                right_pile = pile;
            }
        } else if guess == 3 {
            // exit
            println!("Exiting...");
            break;
        } else if guess < 0 || guess > 2 {
            return Err("Input must be integer between 0 and 2 inclusive");
        } else if guess == 2 {
            // wait for computer to play
            // iterate through all possible playable cards for left and right
            // if you find one you can play, play -> update the correct iterator for the card played
            let computer_card_options = computer_five.get_cards();
            let mut couldnt_play = 0;
            for guess in 0..computer_card_options.len() {
                let chosen_card = computer_card_options.get(guess as usize).unwrap();
                // if the card isn't back it cannot be played
                match play_card(&mut left_pile, &mut right_pile, *chosen_card) {
                    None => {
                        computer_five.remove_card(&chosen_card);
                        match computer_complete_hand.deal_one() {
                            None => {}
                            Some(new_card) => computer_five.add_card(new_card),
                        }
                        if computer_five.len() == 0 {
                            println!("Computer wins! It is out of cards!");
                            return Ok(());
                        }
                        break;
                    }
                    Some(_) => {
                        couldnt_play += 1;
                        continue;
                    }
                }
            }
            // if both can't play any cards -> pull from each player and computers deck
            if couldnt_play == computer_card_options.len() {
                println!("Computer couldn't play any cards!");
            }
        }
        left_card = left_pile.get_top();
        right_card = right_pile.get_top();
    }
    Ok(())
}
