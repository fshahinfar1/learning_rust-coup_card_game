// @author: Farbod Shahinfar
// @date: 5 Feb 2021
//

// use std::io;

mod game_move;
mod hero;
mod player;

use crate::hero::Hero;
use crate::game_move::Move;

fn new_deck() -> [Hero; player::COUNT_CARDS] {
    // Create a deck of cards. When dealing the cards would be assigned to a
    // player and the value of Empty would be placed in the deck.
    let mut deck = [Hero::Empty; player::COUNT_CARDS];
    for r in 0..player::REPEAT_EACH_HERO {
        deck[player::COUNT_HERO*r] = Hero::Assassin;
        deck[player::COUNT_HERO*r + 1] = Hero::Commander;
        deck[player::COUNT_HERO*r + 2] = Hero::Oligarch;
        deck[player::COUNT_HERO*r + 3] = Hero::Princess;
        deck[player::COUNT_HERO*r + 4] = Hero::Ambassador;
    }
    // Return deck
    deck
}

fn main() {
    // TODO: read from input
    // max number of players is 6
    let number_of_players = 3;
    let mut deck = new_deck();
    let mut players = player::prepare_players(number_of_players, &mut deck);

    // Play the game
    let mut round = 0;
    let mut active_players = number_of_players;
    loop {
        // Each player performs a move
        for i in 0..number_of_players {
            // This player has already lost
            if players[i].has_lost() {
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
                            if players[other_index].has_lost() {
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
                        if players[other_index].has_lost() {
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
        if !players[i].has_lost() {
            println!("Winner: {}", players[i].name);
            break;
        }
    }
    println!("End of Game");
}
