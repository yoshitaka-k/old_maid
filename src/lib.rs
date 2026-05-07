/// ババ抜き処理

pub mod cli;
pub mod utils;
pub mod trump;
pub use trump::{GameMode, Field, Deck, Card, Player};

use cli::console::*;
use crate::utils::{rand, dice_role};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;
use std::thread;

//////////////////////////////////////////////////

const MAX_CPU_COUNT: usize = 7;
const DEFAULT_CPU_COUNT: usize = 3;

//////////////////////////////////////////////////

pub fn init_current_player(temp_current: usize, players_count: &usize) -> usize {
    let dice = dice_role();
    (temp_current + (dice - 1)) % players_count
}

pub fn cpu_member_input() -> usize {
    let cpu_count = loop {
        match read_usize_line(&format!(
                "CPU Player Num (Input 1-{}, Default {}): ",
                    MAX_CPU_COUNT, DEFAULT_CPU_COUNT
                ), DEFAULT_CPU_COUNT) {
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

pub fn deal_setup(mode: &GameMode,
        current: &usize,
        players: &mut Vec<Player>,
        field: &mut Field) {
    // Shuffle the cards.
    let mut deck = Deck::new(mode, field);
    let deck_len = deck.len();

    // Deal the cards.
    execute_with_progress(deck_len as u64, "Deal the cards.", "Deal the cards end.", |pb| {
        'deck_deal: for i in 0..deck_len {
            let target_idx = (current + i) % players.len();
            if let Some(card) = deck.draw() {
                players[target_idx].add_hand(card);

                // execute_with_progressのpb
                pb.inc(1);

                // 早すぎるからms待ち
                thread::sleep(Duration::from_millis(rand(50..=100)));
            } else {
                break 'deck_deal;
            }
        }
    });
}

fn _set_progress_spinners(players: &mut Vec<Player>) -> Vec<ProgressBar> {
    let multi = MultiProgress::new();
    let mut spinners = Vec::new();

    for player in players.iter() {
        let pb = multi.add(ProgressBar::new_spinner());

        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["|", "/", "-", "\\"])
        );
        pb.set_message(format!("{} Arrange my Hand (pair off).", player.get_name()));
        spinners.push(pb);
    }

    spinners
}

pub fn arrange_my_hand(players: &mut Vec<Player>, field: &mut Field) {
    let mut all_discards = Vec::new();

    let spinners = _set_progress_spinners(players);

    thread::scope(|s| {
        let mut handles = Vec::new();

        for (player, pb) in players.iter_mut().zip(spinners) {
            let handle = s.spawn(move || {
                let discards = player.discard_all_pairs_same_rank();
                player.sort_hand();

                pb.finish_with_message(format!("{} Arrange my Hand end.", player.get_name()));

                discards
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Ok(pair) = handle.join() {
                all_discards.push(pair);
            }
        }
    });

    for discards in all_discards {
        field.record_discards(discards);
    }
}

//////////////////////////////////////////////////

fn _run_player(players: &Vec<Player>, current: &usize, target_player_idx: &usize) -> usize {
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
            pick_card_idx = rand(0..max_idx);
        } else {
            pick_card_idx = 0;
        }

        let msg = &format!("Draw a card from {}", players[*target_player_idx].get_name());
        execute_with_spinner(msg, "", || {
            // 早すぎるから確認用
            let m = rand(200..300);
            thread::sleep(Duration::from_millis(m));
        });
    }

    pick_card_idx
}

fn _add_rank_player(player: &mut Player, field: &mut Field, rank: usize) {
    player.set_rank(rank);
    field.add_rank(player.clone());
}

//////////////////////////////////////////////////

pub fn run(mode: &GameMode, players: &mut Vec<Player>, field: &mut Field) {
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
                _add_rank_player(&mut players[current], field, rank.clone());

                println!("------------------------------");
                system("Game end.");

                match mode {
                    GameMode::OldMaid => { },
                    GameMode::OldMan => {
                        system(&format!("joker: {}.", field.get_joker()));
                    }
                }

                field.display_rank();

                break 'game_loop;
            }
        }

        // Pick card selected.
        let pick_card_idx = _run_player(&players, &current, &target_player_idx);

        // Pick card.
        let pick_card = players[target_player_idx].remove_hand(pick_card_idx);
        if current == 0 {
            player_info(&format!("pick card: {}", pick_card.get_name()), current == 0);
        }
        players[current].add_hand(pick_card.clone());

        // 引かれて手札無くなったら
        if players[target_player_idx].hand_len() == 0 {
            rank = rank + 1;
            _add_rank_player(&mut players[target_player_idx], field, rank.clone());
        }

        // Pair?
        let pair = players[current].try_discard_pair_same_rank();
        if pair.len() > 0 {
            field.record_discards(pair);

            player_discard_pair_info(&pick_card.get_name(), current == 0);

            if players[current].hand_len() == 0 {
                rank = rank + 1;
                _add_rank_player(&mut players[current], field, rank.clone());
            }
        }

        players[current].update_status_joker();

        player_hand_info(&mut players[current], current == 0);

        current = (current + 1) % players_count;

        println!("------------------------------");
    }
}