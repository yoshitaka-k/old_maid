use std::io::{self, Write};
use console::Style;

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use std::thread;
use rand::Rng;

use crate::Player;

//////////////////////////////////////////////////

pub fn system(prompt: &str) {
    let style = Style::new().green();
    println!("{}", style.apply_to(prompt));
}

pub fn info(prompt: &str) {
    let style = Style::new().cyan();
    println!("{}", style.apply_to(prompt));
}

pub fn error(prompt: &str) {
    let style = Style::new().red();
    println!("{}", style.apply_to(prompt));
}

//////////////////////////////////////////////////

fn player_color(is_human: bool) -> Style {
    if is_human {
        Style::new().green()
    } else {
        Style::new().magenta()
    }
}

pub fn turn_info(turn: &usize, name: &str, is_human: bool) {
    println!("{}", player_color(is_human).apply_to(format!("Turn: {} / {}", turn, name)));
}

pub fn clear_info(rank: &usize, name: &str) {
    println!("{}", Style::new().yellow().apply_to(&format!("{}. cleard {}.", rank, name)));
}

pub fn player_info(prompt: &str, is_human: bool) {
    println!("{}", player_color(is_human).apply_to(prompt));
}

pub fn player_discard_pair(card_name: &String, is_human: bool) {
    player_info(&format!("Discard a pair {}.", card_name), is_human);
}

pub fn player_hand_info(player: &Player, is_human: bool) {
    let name = player.get_name();
    let hand_len = player.hand_len();
    println!("{}", &format!("{} Hand Count: {}", name, hand_len));
    if is_human {
        player.display_hand();
    }
}

//////////////////////////////////////////////////

pub fn cpu_dummy_thinking(name: &str) {
    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["|", "/", "-", "\\"])
    );
    pb.set_message(format!("Draw a card from {}", name));

    // 早すぎるから確認用
    let m = rand::thread_rng().gen_range(200..300);
    thread::sleep(Duration::from_millis(m));

    pb.finish_with_message(format!("Draw end."));
}

//////////////////////////////////////////////////

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    Ok(line.trim().to_string())
}

pub fn read_usize_line(prompt: &str, default: usize) -> io::Result<usize> {
    let line = read_line(prompt)?;

    if line.is_empty() {
        Ok(default)
    } else {
        line.parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
    }
}
