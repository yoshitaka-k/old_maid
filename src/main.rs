use clap::Parser;
use console::Style;
use figlet_rs::FIGlet;

use old_maid::cli::console::{
    print_single_separator,
    print_double_separator,
    print_hr,
    info,
    system,
    system_bold
};
use old_maid::game::{
    cpu_member_input,
    cpu_group_input,
    players_setup,
    game_raound_input,
    deal_setup,
    organize_my_hand_setup,
    init_current_player,
    run,
};
use old_maid::utils::{capitalize, rand_range};
use old_maid::{Field, GameMode};
use old_maid::wait_for_dramatic_pause;

use old_maid::Player;

use old_maid::constants::{
    RANK_1ST_ICON,
    RANK_2ND_ICON,
    RANK_3RD_ICON,
};

//////////////////////////////////////////////////

/// コマンドライン引数
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Game mode: maid or man (Default: maid)
    #[arg(short, long, default_value = "maid")]
    mode: String,
}

//////////////////////////////////////////////////

/// 合計リザルト（総ポイント順。同点は `players` の参加順＝先頭ほど上位）
fn game_result(players: &[Player]) {
    if players.is_empty() {
        return;
    }

    let style_title = Style::new()
        .yellow()
        .bold()
        .apply_to("Total Game Result");
    println!("================ {} ===============", style_title);
    println!("{}", Style::new().dim().apply_to("各ラウンドの獲得ポイント"));

    for player in players {
        print!(
            "  {:<6}: 合計 {:>2} pt ( ",
            player.get_name(),
            player.get_point()
        );
        let point = player.get_history_point();
        let mut pt: String = String::new();
        for p in point {
            pt = format!("{} {:>2} pt", pt, p);
        }
        println!("{} )", pt.trim());
    }

    print_hr('-', 50);

    println!("{}", Style::new().green().apply_to("総合順位（同点は参加順）"));

    let mut order: Vec<usize> = (0..players.len()).collect();
    order.sort_by(|&i, &j| {
        players[j]
            .get_point()
            .cmp(&players[i].get_point())
            .then(i.cmp(&j))
    });

    for (place, &idx) in order.iter().enumerate() {
        let p = &players[idx];
        let medal = match place {
            0 => RANK_1ST_ICON,
            1 => RANK_2ND_ICON,
            2 => RANK_3RD_ICON,
            _ => "  ",
        };
        println!(
            "  {:>2} {} {:<6} — {:>2} pt",
            place + 1,
            medal,
            p.get_name(),
            p.get_point()
        );
    }

    print_hr('=', 50);
}

/// メイン
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mode = match args.mode.as_str() {
        "maid" => GameMode::OldMaid,
        "man" => GameMode::OldMan,
        _ => GameMode::OldMaid,
    };

    // Title
    let style = Style::new().cyan().bold();
    let standard_font = FIGlet::standard().unwrap();
    print!("{}", style.apply_to(standard_font.convert(&format!("Old {}", capitalize(&args.mode))).unwrap()));

    print_single_separator();

    println!("  Version: {}  |  License: {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE"));
    println!("  Starting Old {} Game Engine... 🚀", capitalize(&args.mode));

    print_single_separator();

    system_bold("  Key of Game Force quit. (Ctrl+C or Ctrl+D).");

    print_double_separator();


    // Player setup
    let cpu_member = cpu_member_input();
    info(&format!("  Confirmed: {} CPU Players Joining...", cpu_member));
    wait_for_dramatic_pause();

    print_single_separator();

    let cpu_group = cpu_group_input();
    info(&format!("  CPU Difficulty Preset: {:?}.", cpu_group));
    wait_for_dramatic_pause();

    print_single_separator();

    let mut players = players_setup(cpu_member, &cpu_group);
    let players_count = players.len();
    system(&format!("Players: {} members.", players_count));
    wait_for_dramatic_pause();

    print_single_separator();

    let game_round = game_raound_input();
    info(&format!("  Confirmed: {} Rounds Session.", game_round));
    wait_for_dramatic_pause();

    print_single_separator();

    system(&format!("Game round: {} Rounds Session.", game_round));
    wait_for_dramatic_pause();

    print_single_separator();

    // Temp dice role Player.
    let temp_current = rand_range(0..players_count);
    system(&format!("Pre-Roller: {}", players[temp_current].get_name()));
    wait_for_dramatic_pause();

    // Dice role Player.
    let dice_current = init_current_player(temp_current, players_count);
    system(&format!("Deciding Roller: {}", players[dice_current].get_name()));
    wait_for_dramatic_pause();

    // Start current Player.
    let mut current = init_current_player(dice_current, players_count);
    system(&format!("Starting Dealer: {}", players[current].get_name()));
    wait_for_dramatic_pause();

    print_double_separator();

    let mut round = 0;
    while round < game_round {
        system(&format!("Round start {} / {}", round+1, game_round));

        print_double_separator();

        let mut field = Field::new();

        if round > 0 {
            for player in &mut players {
                player.hand_clear();
                player.status_clear();
                field.clear();
            }
        }

        deal_setup(&mode, current, &mut players, &mut field);

        print_single_separator();

        organize_my_hand_setup(&mut players, &mut field);
        wait_for_dramatic_pause();

        print_single_separator();

        for player in &mut players {
            info(&format!("{:<6} Hand Count: {}", player.get_name(), player.hand_len()));
            wait_for_dramatic_pause();
        }

        run(&mode, round + 1, current, &mut players, &mut field);

        round = round + 1;
        current = (current + 1) % players_count;
    }

    game_result(&players);

    system("Game end.");

    println!("");

    Ok(())
}
