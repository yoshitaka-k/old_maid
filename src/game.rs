use std::thread;
use std::time::Duration;
use rand::prelude::SliceRandom;


use console::Style;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::cli::console::*;
use crate::logic::{Cpu, CpuLevelGroup, Human};
use crate::trump::player::PlayerType;
use crate::utils::{dice_role, rand_range};
use crate::{Deck, Field, GameMode, Player};

use crate::constants::{
    MIN_CPU_COUNT,
    MAX_CPU_COUNT,
    DEFAULT_CPU_COUNT,
    DEFAULT_CPU_LEVEL_GROUP,
    MIN_ROUND_COUNT,
    MAX_ROUND_COUNT,
    DEFAULT_ROUND_COUNT,
    JOKER_TURN_BONUS,
};

use crate::constants::{
    RANK_1ST_ICON,
    RANK_2ND_ICON,
    RANK_3RD_ICON,
};

/// 起家指定
pub fn init_current_player(temp_current: usize, players_count: usize) -> usize {
    let dice = dice_role();
    info(&format!("  ->Dice Result: {}", dice));

    (temp_current + (dice - 1)) % players_count
}

/// 何人のCPUを参加させるか入力
pub fn cpu_member_input() -> usize {
    input_usize_read_line(
        &format!("CPU Player Num (Input {}-{}, Default {})", MIN_CPU_COUNT, MAX_CPU_COUNT, DEFAULT_CPU_COUNT),
        DEFAULT_CPU_COUNT,
        MIN_CPU_COUNT,
        MAX_CPU_COUNT
    )
}

/// CPUの強さグループの選択
pub fn cpu_group_input() -> CpuLevelGroup {
    println!("CPU Strategy level group Input.");
    println!("  1: Beginner, 2: Medium, 3: Veteran, 0: Random Selected.");

    let input = input_usize_read_line(
        &format!("CPU Strategy leve group (Default {})", DEFAULT_CPU_LEVEL_GROUP),
        DEFAULT_CPU_LEVEL_GROUP,
        0, 3
    );

    match input {
        0 => *[
            CpuLevelGroup::Beginner,
            CpuLevelGroup::Medium,
            CpuLevelGroup::Veteran,
        ]
        .choose(&mut rand::thread_rng())
        .unwrap(),
        1 => CpuLevelGroup::Beginner,
        2 => CpuLevelGroup::Medium,
        3 => CpuLevelGroup::Veteran,
        _ => *[
            CpuLevelGroup::Beginner,
            CpuLevelGroup::Medium,
            CpuLevelGroup::Veteran,
        ]
        .choose(&mut rand::thread_rng())
        .unwrap(),
    }
}

/// 参加プレイヤーの初期設定
pub fn players_setup(cpu_count: usize, cpu_group: &CpuLevelGroup) -> Vec<Player> {
    // Player setup.
    let mut players = Vec::new();
    players.push(Player::new(String::from("Player")));

    for i in 1..=cpu_count {
        let mut player = Player::new(format!("CPU {}", i));
        player.set_player_type(PlayerType::Cpu(Cpu::new_level(cpu_group)));

        players.push(player);
    }

    players
}

/// ラウンド数の指定
pub fn game_raound_input() -> usize {
    input_usize_read_line(
        &format!("Game round setup (Input {}-{}, Default {})", MIN_ROUND_COUNT, MAX_ROUND_COUNT, DEFAULT_ROUND_COUNT),
        DEFAULT_ROUND_COUNT,
        MIN_ROUND_COUNT,
        MAX_ROUND_COUNT
    )
}

/// 山札作り
pub fn deck_setup(mode: &GameMode, player: &Player, field: &mut Field) -> Deck {
    // Deck Setting.
    let mut deck = Deck::new(mode);

    // Deck Shuffle.
    execute_with_spinner(
            &format!("Deck setup and {} a shuffle...", player.get_name()),
            &format!("Deck setup and {} a shuffle end.", player.get_name()),
        || {
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
    });

    deck
}

/// 山札配り
/// * `deck` - 山札
/// * `current` - 起家プレイヤー
/// * `players` - 参加プレイヤー達
pub fn deal_setup(deck: &mut Deck, current: usize, players: &mut Vec<Player>) {
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
                thread::sleep(Duration::from_millis(rand_range(10..=100)));
            } else {
                break 'deck_deal;
            }
        }
    });
}

/// 手札並び替え
fn organize_hand(player: &mut Player) {
    if player.has_human() {
        Human::organize_hand(player);
    } else {
        let cpu = Cpu::new();
        cpu.organize_hand(player);
    }
}

/// 配り後の手札整理
/// 処理中スピナーの指定
fn set_progress_spinners(players: &mut Vec<Player>) -> Vec<ProgressBar> {
    let multi = MultiProgress::new();
    let mut spinners = Vec::new();

    for player in players.iter() {
        let pb = multi.add(ProgressBar::new_spinner());

        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["|", "/", "-", "\\"]),
        );
        pb.set_message(format!("{} Arrange my Hand (pair off)...", player.get_name()));
        spinners.push(pb);
    }

    spinners
}

/// 配り後の手札整理
pub fn organize_my_hand_setup(players: &mut Vec<Player>, field: &mut Field) {
    let mut all_discards = Vec::new();

    let spinners = set_progress_spinners(players);

    thread::scope(|s| {
        let mut handles = Vec::new();

        for (player, pb) in players.iter_mut().zip(spinners) {
            let handle = s.spawn(move || {
                let discards = player.discard_all_pairs_same_rank();

                // player hand sort.
                organize_hand(player);

                pb.finish_with_message(format!("{:<6} Arrange my Hand end.", player.get_name()));

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

/// 対象からどの手札を引くか選択
fn run_player(players: &mut Vec<Player>, current: usize, target_player_idx: usize) -> usize {
    let pick_card_idx: usize;

    if players[current].has_human() {
        players[current].display_hand();
        pick_card_idx = Human::choose_card(players, target_player_idx);
    } else {
        let cpu = Cpu::new();
        pick_card_idx = cpu.choose_card(players, current, target_player_idx);

        let msg = &format!("Draw a card from {}...", players[target_player_idx].get_name());
        execute_with_spinner(msg, "", || {
            // 早すぎるから少し待機
            let ms = rand_range(200..300);
            thread::sleep(Duration::from_millis(ms));
        });
    }

    players[current].add_history_choose_index(pick_card_idx);

    pick_card_idx
}

/// 順位決定、ポイント加算
fn add_rank_player(player_count: usize, player: &mut Player, field: &mut Field) {
    let rank = field.get_rank_len();

    player.set_rank(rank + 1);
    player.update_point(player_count - rank);

    player.update_history_joker_turn();

    field.add_rank(player.clone());
}

/// ゲーム実行処理
pub fn run(mode: &GameMode, round: usize, mut current: usize, players: &mut Vec<Player>, field: &mut Field) {
    let players_count = players.len();

    let mut turn = 0;

    print_double_separator();

    'game_loop: loop {
        turn += 1;

        if turn > 300 {
            print_single_separator();

            system("300 Turn over is Process exit.");
            system("Round is Draw end.");

            println!("");

            break 'game_loop;
        }

        let mut target_player_idx = (current + players_count - 1) % players_count;

        let name = format!("{} ({})", players[current].get_name(), players[current].player_type_name());
        turn_info(round, turn, &name, players[current].has_human());

        // Clear Player.
        if players[current].hand_len() == 0 {
            clear_info(players[current].get_rank(), players[current].get_name());

            print_single_separator();

            current = (current + 1) % players_count;
            continue;
        }

        // Game end.
        while players[target_player_idx].hand_is_empty() {
            target_player_idx = (target_player_idx + players_count - 1) % players_count;

            if current == target_player_idx {
                // player clear.
                add_rank_player(players_count, &mut players[current], field);

                print_single_separator();

                match mode {
                    GameMode::OldMaid => {}
                    GameMode::OldMan => {
                        system(&format!("Mystery card: {}.", field.get_mystery_card_name()));
                    }
                }

                println!("");

                round_result(field);

                system("Round end.");

                println!("");

                break 'game_loop;
            }
        }

        // Pick card selected.
        let pick_card_idx = run_player(players, current, target_player_idx);
        players[target_player_idx].add_history_taken_index(pick_card_idx);

        // Pick card.
        let pick_card = players[target_player_idx].remove_hand(pick_card_idx);
        if players[current].has_human() {
            info(&format!("  pick card: {}", pick_card.get_name()));
        }
        players[current].add_hand(pick_card.clone());

        // player clear.
        if players[target_player_idx].hand_len() == 0 {
            add_rank_player(players_count, &mut players[target_player_idx], field);
        }

        // Pair?
        let pair = players[current].try_discard_pair_same_rank();
        if !pair.is_empty() {
            field.record_discards(pair);

            info(&format!("  Discard a pair {}.", pick_card.get_name()));

            // player clear.
            if players[current].hand_len() == 0 {
                add_rank_player(players_count, &mut players[current], field);
            }
        }

        // player hand sort.
        organize_hand(&mut players[current]);

        players[current].update_status_joker_turn();

        player_hand_info(&mut players[current]);

        current = (current + 1) % players_count;

        print_single_separator();
    }
}

/// 各ラウンドの獲得ポイント
fn each_round_points(players: &[Player]) {
    println!("{}", Style::new().dim().apply_to("各ラウンドの獲得ポイント"));

    for player in players {
        print!(
            "  {:<6}: 合計 {:>2} pt ( ",
            player.get_name(),
            player.get_point()
        );
        let point = player.get_history_point();
        let mut pt_str: String = String::new();
        for p in point {
            pt_str = format!("{} {:>2} pt", pt_str, p);
        }
        println!("{} )", pt_str.trim());
    }
}

/// 各ラウンドのジョーカー保持ターン数
fn each_round_joker_turns(players: &[Player]) {
    println!("{}", Style::new().dim().apply_to("各ラウンドのジョーカー保持ターン数"));

    for player in players {
        let turns = player.get_history_joker_turn();
        let turn_sum: usize = turns.iter().sum();

        print!("  {:<6}: 合計 {:>2} turn ( ", player.get_name(), turn_sum);
        let mut turn_str = String::new();
        for turn in turns {
            turn_str = format!("{} {:<2}", turn_str, turn);
        }
        println!("{} )", turn_str.trim());
    }
}

/// ジョーカー保持ターン合計が最小のプレイヤーに +1（同値は全員）
fn joker_turn_bonus_points(players: &[Player]) -> Vec<usize> {
    if players.is_empty() {
        return Vec::new();
    }

    let turn_sums: Vec<usize> = players
        .iter()
        .map(|player| player.get_history_joker_turn().iter().sum())
        .collect();
    let min_turn_sum = turn_sums.iter().copied().min().unwrap_or(0);

    turn_sums
        .iter()
        .map(|sum| if *sum == min_turn_sum { JOKER_TURN_BONUS } else { 0 })
        .collect()
}

/// ジョーカーボーナス表示
fn joker_bonus_points(players: &[Player], bonus_points: &[usize]) {
    println!("{}", Style::new().dim().apply_to("ジョーカーボーナス（保持ターン最小 +1）"));

    for (player, bonus) in players.iter().zip(bonus_points.iter()) {
        println!("  {:<6}: +{} pt", player.get_name(), bonus);
    }
}

/// 総合順位
fn total_rank(players: &[Player], bonus_points: &[usize]) {
    println!("{}", Style::new().green().apply_to("総合順位（同点は参加順）"));

    let mut order: Vec<usize> = (0..players.len()).collect();
    order.sort_by(|&i, &j| {
        let point_i = players[i].get_point() + bonus_points.get(i).copied().unwrap_or(0);
        let point_j = players[j].get_point() + bonus_points.get(j).copied().unwrap_or(0);
        point_j.cmp(&point_i).then(i.cmp(&j))
    });

    for (place, &idx) in order.iter().enumerate() {
        let p = &players[idx];
        let bonus = bonus_points.get(idx).copied().unwrap_or(0);
        let total_point = p.get_point() + bonus;
        let medal = match place {
            0 => RANK_1ST_ICON,
            1 => RANK_2ND_ICON,
            2 => RANK_3RD_ICON,
            _ => "  ",
        };
        println!(
            "  {:>2} {} {:<6} — {:>2} pt (base {:>2} + bonus {:>1})",
            place + 1,
            medal,
            p.get_name(),
            total_point,
            p.get_point(),
            bonus
        );
    }
}

/// 合計リザルト（総ポイント順。同点は `players` の参加順＝先頭ほど上位）
pub fn game_result(players: &[Player]) {
    if players.is_empty() {
        return;
    }

    let style_title = Style::new()
    .yellow()
    .bold()
    .apply_to("Total Game Result");
    println!("================ {} ===============", style_title);

    each_round_points(players);

    print_hr('-', 50);

    each_round_joker_turns(players);

    print_hr('-', 50);

    let bonus_points = joker_turn_bonus_points(players);
    joker_bonus_points(players, &bonus_points);

    print_hr('-', 50);

    total_rank(players, &bonus_points);

    print_hr('=', 50);
}
