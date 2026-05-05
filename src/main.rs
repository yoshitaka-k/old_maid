mod cli;

use cli::console::{read_usize_line, system, info, error, player_info};
use old_maid::{Field, Deck, Player};
use rand::Rng;

fn cpu_member_input() -> usize {
    let cpu_count = loop {
        match read_usize_line("CPU Player Num (Input 1-8): ") {
            Ok(num) if (1..=8).contains(&num) => {
                break num;
            },
            Ok(_) => error("The input is not a number 1-8."),
            Err(_) => error("The input is not a number."),
        }
    };
    cpu_count
}

fn players_setup(cpu_count: usize) -> Vec<Player> {
    system("Player setup.");

    let mut players = Vec::new();
    players.push(Player::new(String::from("Player")));
    for i in 1..=cpu_count {
        players.push(Player::new(format!("CPU {}", i)));
    }
    players
}

fn deal_setup(players: &mut Vec<Player>) {
    system("Shuffle the cards.");
    let mut deck = Deck::new();

    // let deck_len = deck.len();
    // println!("Deck Count: {}", deck_len);

    system("Deal the cards.");
    while deck.len() > 0 {
        for player in players.iter_mut() {
            if let Some(card) = deck.draw() {
                player.hand_in(card);
            } else {
                break;
            }
        }
    }
}

//////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==============================");

    let mut players = players_setup(cpu_member_input());
    let players_count = players.len();
    info(&format!("Players: {} members.", players_count));

    deal_setup(&mut players);

    let mut field = Field::new();

    system("Sort my hand.");
    for player in &mut players {
        player.sort_hand();
    }

    system("Arrange my Hand (pair off).");
    for player in &mut players {
        player.discard_all_pairs_same_rank(&mut field);
    }

    // info(&format!("Table discard (all players, order = time discarded): {:?}", field.all_discards()));

    println!("------------------------------");
    // info("Players hand count.");
    for player in &mut players {
        info(&format!("{} Hand Count: {}", player.get_name(), player.get_hand_len()));
    }

    info(&format!("Player Hand: {:?}", players[0].get_hand_cards()));

    let mut current = 0;
    let mut turn = 0;
    let mut rank = 0;

    println!("==============================");

    'game_loop: loop {
        turn += 1;

        info(&format!("Turn: {} / {}", turn, players[current].get_name()));

        // Player Clear.
        if players[current].get_hand_len() == 0 {
            player_info(&format!("{} of no hand card is cleard.", players[current].get_name()), current);
            current = (current + 1) % players_count;

            println!("------------------------------");
            continue;
        }

        // Game end.
        let mut target_player_idx = (current + players_count - 1) % players_count;
        while players[target_player_idx].hand_is_empty() {
            target_player_idx = (target_player_idx + players_count - 1) % players_count;

            if current == target_player_idx {
                rank = rank + 1;
                players[target_player_idx].set_rank(rank);

                field.set_rank_player_name(players[current].get_name());

                system("Game end.");
                println!("{:?}", field.get_rank());
                info(&format!("Your rank {}", players[0].get_rank()));

                break 'game_loop;
            }
        }

        // Pick card selected.
        let max_idx = players[target_player_idx].get_hand_len().saturating_sub(1);
        let pick_card_idx: usize;

        if current == 0 {
            pick_card_idx = loop {
                match read_usize_line(&format!("Draw a card from CPU {} (index from the left 0-{}): ", target_player_idx, max_idx)) {
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
        }

        // println!("pick card index: {}", pick_card_idx);
        let pick_card = players[target_player_idx].hand_out(pick_card_idx);
        if current == 0 {
            info(&format!("pick card: {}", pick_card));
        }
        players[current].hand_in(pick_card.clone());

        if players[target_player_idx].get_hand_len() == 0 {
            rank = rank + 1;
            players[target_player_idx].set_rank(rank);

            field.set_rank_player_name(players[target_player_idx].get_name());
        }

        if players[current].try_discard_pair_same_rank(&mut field) {
            player_info(&format!("Discard a pair {}.", pick_card), current);

            if players[current].get_hand_len() == 0 {
                rank = rank + 1;
                players[target_player_idx].set_rank(rank);

                field.set_rank_player_name(players[current].get_name());
            }
        }

        player_info(&format!("{} Hand Count: {}", players[current].get_name(), players[current].get_hand_len()), current);
        if current == 0 {
            player_info(&format!("Player Hand: {:?}", players[current].get_hand_cards()), current);
        }

        current = (current + 1) % players_count;

        println!("------------------------------");
    }

    Ok(())
}
