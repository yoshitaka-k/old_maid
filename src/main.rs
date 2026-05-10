use clap::Parser;
use console::Style;
use figlet_rs::FIGlet;

use old_maid::cli::console::{info, system, system_bold};
use old_maid::game::{
    cpu_member_input, deal_setup, init_current_player, organize_my_hand_setup, players_setup, run,
};
use old_maid::utils::{capitalize, rand_range};
use old_maid::{Field, GameMode};

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
    println!("------------------------------");
    println!("  Version: {}  |  License: {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_LICENSE"));
    println!("  Starting Old {} Game Engine... 🚀", capitalize(&args.mode));
    println!("------------------------------");
    system_bold("  Key of Game Force quit. (Ctrl+C or Ctrl+D).");
    println!("==============================");

    let cpu_member = cpu_member_input();
    let mut players = players_setup(cpu_member);
    let players_count = players.len();

    system(&format!("Players: {} members.", players_count));

    println!("------------------------------");

    // Temp dice role Player.
    let temp_current = rand_range(0..players_count);
    system(&format!("Pre-Roller: {}", players[temp_current].get_name()));

    // Dice role Player.
    let dice_current = init_current_player(temp_current, players_count);
    system(&format!("Deciding Roller: {}", players[dice_current].get_name()));

    // Start current Player.
    let current = init_current_player(dice_current, players_count);
    system(&format!("Starting Dealer: {}", players[current].get_name()));

    println!("------------------------------");

    let mut field = Field::new();

    deal_setup(&mode, current, &mut players, &mut field);

    println!("------------------------------");

    organize_my_hand_setup(&mut players, &mut field);

    println!("------------------------------");
    for player in &mut players {
        info(&format!("{} Hand Count: {}", player.get_name(), player.hand_len()));
    }

    println!("------------------------------");
    players[0].display_hand();

    run(&mode, &mut players, &mut field);

    Ok(())
}
