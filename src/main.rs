use std::{fmt, io};
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
    rank: Rank,
}

#[derive(Copy, Clone, PartialEq)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
const RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

impl fmt::Display for Rank {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Rank::Ace => "Ace",
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
        };
        write!(fmt, "{}", s)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} of {}\n", self.rank, self.suite)
    }
}

impl fmt::Display for Suite {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Suite::Clubs => "Clubs",
            Suite::Hearts => "Hearts",
            Suite::Spades => "Spades",
            Suite::Diamonds => "Diamonds",
        };
        write!(fmt, "{}", s)
    }
}

impl Card {
    fn new(suite: Suite, rank: Rank) -> Card {
        Card {
            suite: suite,
            rank: rank,
        }
    }
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
#[derive(Clone, PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let cards = Vec::<Card>::new();
        Deck { cards: cards }
    }

    fn init(&mut self) {
        if self.cards.is_empty() {
            for rank in RANKS {
                self.cards.push(Card::new(Suite::Diamonds, rank));
                self.cards.push(Card::new(Suite::Spades, rank));
                self.cards.push(Card::new(Suite::Clubs, rank));
                self.cards.push(Card::new(Suite::Hearts, rank));
            }
        }
    }

    fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal_one(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn deal_hand(&mut self, num_of_cards: usize) -> Option<Deck> {
        if num_of_cards > self.cards.len() {
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

    fn len(&self) -> usize {
        self.cards.len()
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
    idx: isize,
    top: Card,
}

impl<'a> Pile {
    fn new(card: &Card) -> Pile {
        Pile {
            idx: RANKS.iter().position(|&r| r == card.rank).unwrap() as isize,
            top: *card,
        }
    }
    // put a card on top of the current one
    // return None if card has been placed
    fn place_card(&mut self, card: Card, who: &str) -> Option<Card> {
        let rank = card.rank;
        let above = (self.idx + 1).rem_euclid(RANKS.len() as isize);
        let below = (self.idx - 1).rem_euclid(RANKS.len() as isize);
        if RANKS[above as usize] == rank {
            println!("{} placed {} on top of {}", who, card, self.top);
            self.top = card;
            self.idx = above;
            None
        } else if RANKS[below as usize] == rank {
            println!("{} placed {} on top of {}", who, card, self.top);
            self.top = card;
            self.idx = below;
            None
        } else {
            Some(card)
        }
    }

    fn set_card(&mut self, card: Card) {
        self.idx = RANKS.iter().position(|&r| r == card.rank).unwrap() as isize;
        self.top = card;
    }

    fn can_place_card(&self, card: Card) -> bool {
        (RANKS[(self.idx - 1).rem_euclid(RANKS.len() as isize) as usize] == card.rank)
            || (RANKS[(self.idx + 1).rem_euclid(RANKS.len() as isize) as usize] == card.rank)
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

fn play_card(
    left_pile: &mut Pile,
    right_pile: &mut Pile,
    chosen_card: Card,
    who: &str,
) -> Option<Card> {
    match left_pile.place_card(chosen_card, who) {
        None => None,
        Some(card) => match right_pile.place_card(card, who) {
            None => None,
            Some(card) => Some(card),
        },
    }
}

fn play_card_pile(pile: &mut Pile, chosen_card: Card, who: &str) -> Option<Card> {
    match pile.place_card(chosen_card, who) {
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
            match play_card_pile(&mut pile, *chosen_card, "You") {
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
                    println!("Can't place {} on top of {}", *chosen_card, pile.get_top());
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
            for chosen_card in computer_card_options.iter() {
                // if the card isn't back it cannot be played
                match play_card(&mut left_pile, &mut right_pile, *chosen_card, "Computer") {
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
                let mut num_of_cards_player_cant_play = 0;
                // check if user can play cards
                for user_card in player_five.get_cards().iter() {
                    if !left_pile.can_place_card(*user_card)
                        && !right_pile.can_place_card(*user_card)
                    {
                        num_of_cards_player_cant_play += 1;
                    }
                }
                // if the player and computer both can't play any cards
                // big one from each of their decks to place
                if num_of_cards_player_cant_play == player_five.len() {
                    println!("Neither you nor the computer could play any cards");
                    println!("So both of you give up a card and put it into the middle two");
                    if player_complete_hand.len() > 0 {
                        left_pile.set_card(player_complete_hand.deal_one().unwrap());
                    } else {
                        if player_five.len() > 0 {
                            left_pile.set_card(player_five.deal_one().unwrap());
                        } else {
                            println!("You win! Game over!");
                        }
                    }
                    if player_complete_hand.len() > 0 {
                        right_pile.set_card(computer_complete_hand.deal_one().unwrap());
                    } else {
                        if player_five.len() > 0 {
                            right_pile.set_card(computer_five.deal_one().unwrap());
                        } else {
                            println!("Computer wins! Game over!");
                        }
                    }
                }
            }
        }
        left_card = left_pile.get_top();
        right_card = right_pile.get_top();
    }
    Ok(())
}
