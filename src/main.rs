// @author: Farbod Shahinfar
// @date: 5 Feb 2021

use std::io;
use rand::Rng;

// Define some constants
const COUNT_HERO: usize = 5;
const REPEAT_EACH_HERO: usize = 3;
const COUNT_CARDS: usize = COUNT_HERO * REPEAT_EACH_HERO;
const COUNT_PLAYER_CARDS: usize = 2;
const MAX_NUM_PLAYER: usize = 6;

enum Move {
    TakeOneCoin,    // Everyone can play this
    TakeThreeCoin,  // Only Oligarch can play this
    TakeTaxFrom,    // Only Commander can play this
    Assassinate,    // Only Assassin can play this
    ChangeCards,    // Only Ambassador can play this
    Coup,           // Everyone who has 7 coins or more
}

impl Move {
    fn name(&self) -> &str {
        match self {
            Move::TakeOneCoin => {
                "take one coin"
            },
            Move::TakeThreeCoin => {
                "take three coin"
            },
            Move::TakeTaxFrom => {
                "take tax from"
            },
            Move::Assassinate => {
                "assassinate"
            },
            Move::ChangeCards => {
                "change card"
            },
            Move::Coup => {
                "coup!"
            },
        }
    }
}

// Define some types
#[derive(Copy, Clone, PartialEq)]
enum Hero {
    Empty,
    Assassin,
    Commander,
    Oligarch,
    Princess,
    Ambassador,
}

impl Hero {
    fn get_name(&self) -> &str {
        // TODO: match and return a string
        match self {
            Hero::Empty => "Empty",
            Hero::Assassin => "Assassin",
            Hero::Commander => "Commander",
            Hero::Oligarch => "Oligarch",
            Hero::Princess => "Princess",
            Hero::Ambassador => "Ambassador",
        }
    }
}

const NAMES: [&str; MAX_NUM_PLAYER] = ["Red", "Blu", "Violate", "Bianco", "Nero", "Cyan"];

#[derive(Copy, Clone)]
struct Player<'a> {
    name: & 'a str,
    cards: [Hero; COUNT_PLAYER_CARDS],
    count_card: u32,
    coin: u32,
    index: usize,
}

impl Player<'_> {
    fn show_hand(&self) {
        // TODO: print to the console this players cards.
        println!("Player: {} (coin: {})", self.name, self.coin);
        for i in 0..COUNT_PLAYER_CARDS {
            println!("card #{}: {}", i, self.cards[i].get_name());
        }
        println!("");
    }

    fn has_card(&self, card: Hero) -> bool {
        for i in 0..COUNT_PLAYER_CARDS {
            if card == self.cards[i] {
                return true;
            }
        }
        // return false
        false
    }

    fn lose_card(&mut self) {
        // TODO: use strategy for selecting the dead card
        if let Hero::Empty = self.cards[0] {
            self.cards[1] = Hero::Empty;
        } else {
            self.cards[0] = Hero::Empty;
        }
        self.count_card -= 1;
    }

    fn turn(&self, all_players: &[Player; MAX_NUM_PLAYER], count_player: usize) -> (Move, Option<usize>) {
        // TODO: add deceit to the players strategy

        if self.coin >= 10 {
            // When having at least 10 coin, one must perform a coup!
            // Select a player, TODO: use strategy
            let index = randomly_select_player_with_card(self.index, all_players, count_player);
            // coup against this player
            return (Move::Coup, Some(index));
        } else if self.coin >= 7 {
            // When having 7 coins or more one can coup!
            let should_coup = rand::thread_rng().gen_range(0..2);
            if should_coup == 1 {
                // Select a player, TODO: use strategy
                let index = randomly_select_player_with_card(self.index, all_players, count_player);
                // coup against this player
                return (Move::Coup, Some(index));
            }
        }

        // TODO: use strategy for selecting a card
        let mut selected_card = self.cards[0];
        if let Hero::Empty = selected_card {
            selected_card = self.cards[1];
        }

        match selected_card {
            Hero::Empty => {
                // This should never happen
                return (Move::TakeOneCoin, None);
            },
            Hero::Assassin => {
                if self.coin > 3 {
                    let index = randomly_select_player_with_card(self.index, all_players, count_player);
                    return (Move::Assassinate, Some(index));
                }
                return (Move::TakeOneCoin, None);
            },
            Hero::Commander => {
                // TODO: Try to take tax
                let index = randomly_select_player_with_card(self.index, all_players, count_player);
                return (Move::TakeTaxFrom, Some(index));
            }
            Hero::Oligarch => {
                return (Move::TakeThreeCoin, None);
            },
            Hero::Princess => {
                return (Move::TakeOneCoin, None);
            },
            Hero::Ambassador => {
                return (Move::ChangeCards, None);
            },
        }
    }
}

fn randomly_select_player_with_card(self_index: usize, all_players: &[Player; MAX_NUM_PLAYER], count_player: usize) -> usize {
    // Select a player, TODO: use strategy
    loop {
        let index = rand::thread_rng().gen_range(0..count_player);
        if index != self_index && all_players[index].count_card > 0 {
            return index;
        }
    }
}

fn new_deck() -> [Hero; COUNT_CARDS] {
    // Create a deck of cards. When dealing the cards would be assigned to a
    // player and the value of Empty would be placed in the deck.
    let mut deck = [Hero::Empty; COUNT_CARDS];
    for r in 0..REPEAT_EACH_HERO {
        deck[COUNT_HERO*r] = Hero::Assassin;
        deck[COUNT_HERO*r + 1] = Hero::Commander;
        deck[COUNT_HERO*r + 2] = Hero::Oligarch;
        deck[COUNT_HERO*r + 3] = Hero::Princess;
        deck[COUNT_HERO*r + 4] = Hero::Ambassador;
    }
    // Return deck
    deck
}

fn prepare_players(number_of_players: usize, deck: &mut[Hero; COUNT_CARDS]) -> [Player; MAX_NUM_PLAYER]
{
    // Create a list of players and random deal each of them some cards
    let mut players = [Player {
        name: NAMES[0],
        cards: [Hero::Empty, Hero::Empty],
        count_card: 2,
        coin: 2,
        index: 0,
    }; MAX_NUM_PLAYER];

    // Deal cards
    for index in 0..number_of_players {
        // For each player, select two cards
        players[index].name = NAMES[index];
        players[index].index = index;
        for card_index in 0..COUNT_PLAYER_CARDS {
            loop {
                // Select an index in random. if in the deck the index is not
                // empty then give the card to this player. else try again.
                let deck_index = rand::thread_rng().gen_range(0..COUNT_CARDS);
                match deck[deck_index] {
                    Hero::Empty => continue,
                    _ => {
                        // Found a card
                        players[index].cards[card_index] = deck[deck_index];
                        deck[deck_index] = Hero::Empty;
                        break;
                    }
                }
            }
        }
        // Show what hand this player has
        players[index].show_hand();
    }
    // Return players
    players
}

fn main() {
    // TODO: read from input
    // max number of players is 6
    let number_of_players = 3;
    let mut deck = new_deck();
    let mut players = prepare_players(number_of_players, &mut deck);

    // Play the game
    let mut round = 0;
    let mut active_players = number_of_players;
    loop {
        // Each player performs a move
        for i in 0..number_of_players {
            // This player has already lost
            if players[i].count_card == 0 {
                continue;
            }

            // let player = &mut players[i];
            let (_move, opt_other_index) = players[i].turn(&players, number_of_players);
            println!("{} move: {}", players[i].name, _move.name());
            match _move {
                Move::TakeOneCoin => {
                    players[i].coin += 1;
                },
                Move::TakeThreeCoin => {
                    // TODO: what if someone interrupt...
                    players[i].coin += 3;
                },
                Move::TakeTaxFrom => {
                    if let Some(other_index) = opt_other_index {
                        // let &mut other = &mut players[other_index];
                        // Check if other does not have Commander or Ambassador
                        if !players[other_index].has_card(Hero::Commander) && !players[other_index].has_card(Hero::Ambassador) {
                            let tax = if players[other_index].coin > 2 {2} else {players[other_index].coin};
                            players[i].coin += tax;
                            players[other_index].coin -= tax;
                        }
                    } else {
                        // This error!
                        println!("Error, i do not know exception yet!");
                    }
                },
                Move::Assassinate => {
                    if let Some(other_index) = opt_other_index {
                        // let &mut other = &mut players[other_index];
                        // Check if other does not have Commander or Ambassador
                        if !players[other_index].has_card(Hero::Princess) {
                            players[other_index].lose_card();
                            if players[other_index].count_card == 0 {
                                active_players -= 1;
                            }
                        }
                    } else {
                        // This error!
                        println!("Error, i do not know exception yet!");
                    }
                },
                Move::ChangeCards => {
                    // TODO: implement this
                },
                Move::Coup => {
                    if let Some(other_index) = opt_other_index {
                        // let &mut other = &mut players[other_index];
                        players[other_index].lose_card();
                        if players[other_index].count_card == 0 {
                            active_players -= 1;
                        }
                    } else {
                        // This error!
                        println!("Error, i do not know exception yet!");
                    }
                },
            }
        }

        // Show state of each player at the beginning of each round.
        round += 1;
        println!("=== round: {} ===", round);
        for i in 0..number_of_players {
            players[i].show_hand();
        }

        if active_players == 1 {
            break;
        }
    }

    println!("==============================");
    for i in 0..number_of_players {
        if players[i].count_card > 0 {
            println!("Winner: {}", players[i].name);
            break;
        }
    }
    println!("End of Game");
}
