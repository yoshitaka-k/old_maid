use std::io::{self, Write};
use console::Style;

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

fn player_color(current: usize) -> Style {
    if current == 0 {
        Style::new().green()
    } else {
        Style::new().magenta()
    }
}

pub fn player_info(prompt: &str, current: usize) {
    println!("{}", player_color(current).apply_to(prompt));
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

pub fn read_usize_line(prompt: &str) -> io::Result<usize> {
    let line = read_line(prompt)?;

    line.trim()
        .parse::<usize>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}
