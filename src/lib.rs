use std::time::Duration;
use std::thread;
use indicatif::{ MultiProgress, ProgressBar, ProgressStyle };

pub mod cli;
pub mod utils;
pub mod trump;
pub use trump::{ GameMode, Field, Deck, Card, Player };
use crate::trump::player::PlayerType;

pub mod logic;
pub use logic::{ Human, Cpu, CpuLevelGroup, CpuLevel };

use cli::console::*;
use crate::utils::{ rand_range, dice_role };

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
        let mut player = Player::new(format!("CPU {}", i));
        player.set_player_type(PlayerType::Cpu(Cpu::new_level(CpuLevelGroup::Beginner)));

        players.push(player);
    }

    players
}

fn _organize_hand(player: &mut Player) {
    if player.has_human() {
        Human::organize_hand(player);
    } else {
        let cpu = Cpu::new();
        cpu.organize_hand(player);
    }
}

/// 山札作り
fn _deck_setup(mode: &GameMode, player: &Player, field: &mut Field) -> Deck {
    // Deck Setting.
    let mut deck = Deck::new(mode);

    if deck.has_mystery_card() {
        field.set_mystery_card(deck.pop_mystery_card());
    }

    // Shuffle the cards.
    if player.has_human() {
        Human::deck_shuffle(deck.get_cards());
    } else {
        let cpu = Cpu::new();
        cpu.deck_shuffle(player, deck.get_cards());
    }

    deck
}

/// 山札配り
/// * `mode` - ババ抜き、ジジ抜き
/// * `current` - 起家プレイヤー
/// * `players` - 参加プレイヤー達
/// * `field` - ゲームフィールド情報
pub fn deal_setup(mode: &GameMode, current: &usize, players: &mut Vec<Player>, field: &mut Field) {
    // Deck Setting.
    let mut deck = _deck_setup(&mode, &players[*current], field);
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
                thread::sleep(Duration::from_millis(rand_range(50..=100)));
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

pub fn organize_my_hand(players: &mut Vec<Player>, field: &mut Field) {
    let mut all_discards = Vec::new();

    let spinners = _set_progress_spinners(players);

    thread::scope(|s| {
        let mut handles = Vec::new();

        for (player, pb) in players.iter_mut().zip(spinners) {
            let handle = s.spawn(move || {
                let discards = player.discard_all_pairs_same_rank();

                // player hand sort.
                _organize_hand(player);

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

fn _run_player(players: &mut Vec<Player>, current: &usize, target_player_idx: &usize) -> usize {
    let pick_card_idx: usize;

    if players[*current].has_human() {
        pick_card_idx = Human::input_choose_index(players, target_player_idx);
    } else {
        let cpu = Cpu::new();
        pick_card_idx = cpu.choose_card(players, current, target_player_idx);

        let msg = &format!("Draw a card from {}", players[*target_player_idx].get_name());
        execute_with_spinner(msg, "", || {
            // 早すぎるから少し待機
            let ms = rand_range(200..300);
            thread::sleep(Duration::from_millis(ms));
        });
    }

    players[*current].add_history_choose_index(pick_card_idx);

    pick_card_idx
}

fn _add_rank_player(player: &mut Player, field: &mut Field) {
    field.add_rank(player.clone());

    let rank = field.get_rank_len();
    player.set_rank(rank + 1);
}

//////////////////////////////////////////////////

pub fn run(mode: &GameMode, players: &mut Vec<Player>, field: &mut Field) {
    let players_count = players.len();

    let mut turn = 0;
    let mut current = 0;

    println!("==============================");

    'game_loop: loop {
        turn += 1;
        let mut target_player_idx = (current + players_count - 1) % players_count;

        turn_info(&turn, players[current].get_name(), players[current].has_human());

        // Clear Player.
        if players[current].hand_len() == 0 {
            clear_info(players[current].get_rank(), players[current].get_name());

            println!("------------------------------");

            current = (current + 1) % players_count;
            continue;
        }

        // Game end.
        while players[target_player_idx].hand_is_empty() {
            target_player_idx = (target_player_idx + players_count - 1) % players_count;

            if current == target_player_idx {
                // player clear.
                _add_rank_player(&mut players[current], field);

                println!("------------------------------");
                system("Game end.");

                match mode {
                    GameMode::OldMaid => { },
                    GameMode::OldMan => {
                        system(&format!("Mystery card: {}.", field.get_mystery_card_name()));
                    }
                }

                field.display_rank();

                break 'game_loop;
            }
        }

        // Pick card selected.
        let pick_card_idx = _run_player(players, &current, &target_player_idx);

        // Pick card.
        let pick_card = players[target_player_idx].remove_hand(pick_card_idx);
        if players[current].has_human() {
            player_info(&format!("pick card: {}", pick_card.get_name()), players[current].has_human());
        }
        players[current].add_hand(pick_card.clone());

        // player clear.
        if players[target_player_idx].hand_len() == 0 {
            _add_rank_player(&mut players[target_player_idx], field);
        }

        // Pair?
        let pair = players[current].try_discard_pair_same_rank();
        if pair.len() > 0 {
            field.record_discards(pair);

            player_discard_pair_info(&pick_card.get_name(), players[current].has_human());

            // player clear.
            if players[current].hand_len() == 0 {
                _add_rank_player(&mut players[current], field);
            }
        }

        players[current].update_status_joker_turn();

        player_hand_info(&mut players[current]);

        current = (current + 1) % players_count;

        println!("------------------------------");
    }
}