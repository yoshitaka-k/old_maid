use clap::Parser;
use console::Style;
use figlet_rs::FIGlet;

use old_maid::cli::console::{
    print_single_separator,
    print_double_separator,
    info,
    system,
    system_bold,
};
use old_maid::game::{
    cpu_member_input,
    cpu_group_input,
    players_setup,
    game_raound_input,
    deck_setup,
    deal_setup,
    organize_my_hand_setup,
    init_current_player,
    run,
    game_result,
};
use old_maid::utils::{capitalize, rand_range};
use old_maid::{Field, GameMode};
use old_maid::wait_for_dramatic_pause;

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

    // Round game start.
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

        // Deck Setting.
        let mut deck = deck_setup(&mode, &players[current], &mut field);

        // Card deal.
        deal_setup(&mut deck, current, &mut players);

        print_single_separator();

        // Organize my Hand card.
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

    game_result(&mode, &players);

    system("Game end.");

    println!("");

    Ok(())
}
