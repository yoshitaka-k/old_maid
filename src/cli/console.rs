/// ターミナルに表示させるもの

use std::time::Duration;

use std::io::{ self };
use indicatif::{ MultiProgress, ProgressBar, ProgressStyle };
use rustyline::error::ReadlineError;
// use rustyline::{ DefaultEditor, Result };

use console::Style;
use crate::Player;

//////////////////////////////////////////////////

/// 主にシステム向けな表示
pub fn system(prompt: &str) {
    let style = Style::new().green();
    println!("{}", style.apply_to(prompt));
}

/// 主にシステム向けな表示（太字）
pub fn system_bold(prompt: &str) {
    let style = Style::new().green().bold();
    println!("{}", style.apply_to(prompt));
}

/// 主にお知らせ向けな表示（太字）
pub fn info(prompt: &str) {
    let style = Style::new().cyan();
    println!("{}", style.apply_to(prompt));
}

/// 主にエラー向けな表示
pub fn error(prompt: &str) {
    let style = Style::new().red();
    println!("{}", style.apply_to(prompt));
}

//////////////////////////////////////////////////

/// 人、CPUで色分け
fn player_color(is_human: bool) -> Style {
    if is_human {
        Style::new().green()
    } else {
        Style::new().magenta()
    }
}

/// ターン情報
pub fn turn_info(turn: usize, name: &str, is_human: bool) {
    println!("{}", player_color(is_human).apply_to(format!("Turn: {} / {}", turn, name)));
}

/// 上がりお知らせ
pub fn clear_info(rank: usize, name: &str) {
    println!("{}", Style::new().yellow().apply_to(&format!("{}. cleard {}.", rank, name)));
}

/// 手札情報
pub fn player_hand_info(player: &Player) {
    let name = player.get_name();
    let hand_len = player.hand_len();
    println!("{}", &format!("{} Hand Count: {}", name, hand_len));
    if player.has_human() {
        player.display_hand();
    }
}

//////////////////////////////////////////////////

/// 入力処理
///指定範囲内の値が入力されるまで表示
pub fn input_usize_read_line(input_msg: &str,
                        err_msg: &str,
                        default_num: usize,
                        min_num: usize,
                        max_num: usize) -> usize {
    loop {
        match read_usize_line(
            input_msg,
            default_num,
        ) {
            Ok(num) if (min_num..=max_num).contains(&num) => {
                break num;
            }
            Ok(_) => error(err_msg),
            Err(_) => error("The input is not a number."),
        }
    }
}

/// 入力処理（ベース）
fn read_line(prompt: &str) -> rustyline::Result<String> {
    let mut rl = rustyline::DefaultEditor::new()?;
    let readline = rl.readline(&format!("{}", prompt));

    match readline {
        Ok(line) => Ok(line.trim().to_string()),
        Err(ReadlineError::Interrupted) => {
            system_bold("Pressing Ctrl+C. Ends the Game.");
            std::process::exit(0);
        },
        Err(ReadlineError::Eof) => {
            system_bold("Pressing Ctrl+D. Ends the Game.");
            std::process::exit(0);
        },
        Err(e) => Err(e),
    }
}

/// 入力したのを数値に変換
pub fn read_usize_line(prompt: &str, default: usize) -> rustyline::Result<usize> {
    let line = read_line(prompt)?;

    if line.is_empty() {
        Ok(default)
    } else {
        Ok(line.parse::<usize>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?)
    }
}

//////////////////////////////////////////////////

/// 待ち中表示（スピナー）
pub fn execute_with_spinner<T, F>(set_message: &str, finish_message: &str, f: F) -> T
    where
        F: FnOnce() -> T {
    let mult = MultiProgress::new();
    let pb = mult.add(ProgressBar::new_spinner());

    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["|", "/", "-", "\\"])
    );
    pb.set_message(set_message.to_string());

    let result = f();

    if finish_message.trim().len() > 0 {
        // 終わったらメッセージを変える
        pb.finish_with_message(finish_message.to_string());
    } else {
        // 終わったらバーを消す
        pb.finish_and_clear();
    }

    result
}

/// 待ち中表示（プログレスバー）
pub fn execute_with_progress<T, F>(total: u64, set_message: &str, finish_message: &str, f: F) -> T
    where
        F: FnOnce(&ProgressBar) -> T {
    let mult = MultiProgress::new();
    let pb = mult.add(ProgressBar::new(total));

    pb.set_style(
        ProgressStyle::with_template("{msg} [{wide_bar:.green}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message(set_message.to_string());

    let result = f(&pb);

    if finish_message.trim().len() > 0 {
        // 終わったらメッセージを変える
        pb.finish_with_message(finish_message.to_string());
    } else {
        // 終わったらバーを消す
        pb.finish_and_clear();
    }

    result
}
