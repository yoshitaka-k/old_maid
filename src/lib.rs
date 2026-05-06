//! ババ抜きのゲームロジック（バイナリは `main.rs` からこのクレートを利用する）。
pub mod cli;
pub mod trump;
pub use trump::{Field, Deck, Card, Player};

use cli::console::*;
use rand::Rng;

//////////////////////////////////////////////////

const MAX_CPU_COUNT: usize = 7;
const DEFAULT_CPU_COUNT: usize = 3;

//////////////////////////////////////////////////

pub fn cpu_member_input() -> usize {
    let cpu_count = loop {
        match read_usize_line(&format!("CPU Player Num (Input 1-{}, Default {}): ", MAX_CPU_COUNT, DEFAULT_CPU_COUNT), DEFAULT_CPU_COUNT) {
            Ok(num) if (1..=MAX_CPU_COUNT).contains(&num) => {
                break num;
            },
            Ok(_) => error(&format!("The input is not a number 1-{}.", MAX_CPU_COUNT)),
            Err(_) => error("The input is not a number."),
        }
    };
    cpu_count
}

pub fn players_setup(cpu_count: usize) -> Vec<Player> {
    // Player setup.

    let mut players = Vec::new();
    players.push(Player::new(String::from("Player")));
    for i in 1..=cpu_count {
        players.push(Player::new(format!("CPU {}", i)));
    }
    players
}

pub fn deal_setup(players: &mut Vec<Player>) {
    // Shuffle the cards.
    let mut deck = Deck::new();

    // Deal the cards.
    while deck.len() > 0 {
        for player in players.iter_mut() {
            if let Some(card) = deck.draw() {
                player.add_hand(card);
            } else {
                break;
            }
        }
    }
}

//////////////////////////////////////////////////

fn run_player(players: &Vec<Player>, current: &usize, target_player_idx: &usize) -> usize {
    let max_idx = players[*target_player_idx].hand_len().saturating_sub(1);
    let pick_card_idx: usize;

    if *current == 0 {
        pick_card_idx = loop {
            match read_usize_line(&format!(
                        "Draw a card from {} (index from the left 0-{}, Default 0): ",
                        players[*target_player_idx].get_name(),
                        max_idx
                    ), 0) {
                Ok(num) if (0..=max_idx).contains(&num) => {
                    break num;
                },
                Ok(_) => error(&format!("The input is not a number 0-{}.", max_idx)),
                Err(_) => error("The input is not a number."),
            }
        };
    } else {
        if max_idx > 0 {
            pick_card_idx = rand::thread_rng().gen_range(0..max_idx);
        } else {
            pick_card_idx = 0;
        }
        cpu_dummy_thinking(players[*target_player_idx].get_name());
    }

    pick_card_idx
}

fn add_rank_player(player: &mut Player, field: &mut Field, rank: usize) {
    player.set_rank(rank);
    field.add_rank(player.clone());
}

pub fn run(players: &mut Vec<Player>, field: &mut Field) {
    let players_count = players.len();

    let mut turn = 0;
    let mut current = 0;
    let mut rank = 0;

    println!("==============================");

    'game_loop: loop {
        turn += 1;
        let mut target_player_idx = (current + players_count - 1) % players_count;

        turn_info(&turn, players[current].get_name(), current == 0);

        // Player Clear.
        if players[current].hand_len() == 0 {
            clear_info(players[current].get_rank(), players[current].get_name());
            current = (current + 1) % players_count;

            println!("------------------------------");
            continue;
        }

        // Game end.
        while players[target_player_idx].hand_is_empty() {
            target_player_idx = (target_player_idx + players_count - 1) % players_count;

            if current == target_player_idx {
                rank = rank + 1;
                add_rank_player(&mut players[current], field, rank.clone());

                system("Game end.");
                println!("------------------------------");

                field.display_rank();

                break 'game_loop;
            }
        }

        // Pick card selected.
        let pick_card_idx = run_player(&players, &current, &target_player_idx);

        // Pick card.
        let pick_card = players[target_player_idx].remove_hand(pick_card_idx);
        if current == 0 {
            player_info(&format!("pick card: {}", pick_card.get_name()), current == 0);
        }
        players[current].add_hand(pick_card.clone());

        if players[target_player_idx].hand_len() == 0 {
            rank = rank + 1;
            add_rank_player(&mut players[target_player_idx], field, rank.clone());
        }

        if players[current].try_discard_pair_same_rank(field) {
            player_discard_pair(&pick_card.get_name(), current == 0);

            if players[current].hand_len() == 0 {
                rank = rank + 1;
                add_rank_player(&mut players[current], field, rank.clone());
            }
        }

        players[current].update_status_joker();

        player_hand_info(&mut players[current], current == 0);

        current = (current + 1) % players_count;

        println!("------------------------------");
    }
}