use cli::console::{system, info};
use old_maid::*;

//////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==============================");

    let cpu_member = cpu_member_input();
    let mut players = players_setup(cpu_member);
    let players_count = players.len();

    system(&format!("Players: {} members.", players_count));

    deal_setup(&mut players);

    let mut field = Field::new();

    for player in &mut players {
        player.discard_all_pairs_same_rank(&mut field);
        player.sort_hand();
    }

    println!("------------------------------");
    for player in &mut players {
        info(&format!("{} Hand Count: {}", player.get_name(), player.hand_len()));
    }

    println!("------------------------------");
    players[0].display_hand();

    run(&mut players, &mut field);

    Ok(())
}
