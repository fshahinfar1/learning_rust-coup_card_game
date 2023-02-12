use rand::Rng;
use crate::hero::Hero;
use crate::game_move::Move;

// Define some constants
pub const COUNT_HERO: usize = 5;
pub const REPEAT_EACH_HERO: usize = 3;
pub const COUNT_CARDS: usize = COUNT_HERO * REPEAT_EACH_HERO;
pub const COUNT_PLAYER_CARDS: usize = 2;
pub const MAX_NUM_PLAYER: usize = 6;
pub const NAMES: [&str; MAX_NUM_PLAYER] = ["Red", "Blu", "Violate", "Bianco", "Nero", "Cyan"];

#[derive(Copy, Clone)]
pub struct Player<'a> {
    pub name: & 'a str,
    cards: [Hero; COUNT_PLAYER_CARDS],
    count_card: u32,
    pub coin: u32,
    index: usize,
}

impl Player<'_> {
    pub fn has_lost(&self) -> bool {
        self.count_card == 0
    }

    pub fn show_hand(&self) {
        // TODO: print to the console this players cards.
        println!("Player: {} (coin: {})", self.name, self.coin);
        for i in 0..COUNT_PLAYER_CARDS {
            println!("card #{}: {}", i, self.cards[i].get_name());
        }
        println!("");
    }

    pub fn has_card(&self, card: Hero) -> bool {
        for i in 0..COUNT_PLAYER_CARDS {
            if card == self.cards[i] {
                return true;
            }
        }
        // return false
        false
    }

    pub fn lose_card(&mut self) {
        // TODO: use strategy for selecting the dead card
        if let Hero::Empty = self.cards[0] {
            self.cards[1] = Hero::Empty;
        } else {
            self.cards[0] = Hero::Empty;
        }
        self.count_card -= 1;
    }

    pub fn turn(&self, all_players: &[Player; MAX_NUM_PLAYER], count_player: usize) -> (Move, Option<usize>) {
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

pub fn prepare_players(number_of_players: usize, deck: &mut[Hero; COUNT_CARDS]) -> [Player; MAX_NUM_PLAYER]
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
