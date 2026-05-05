//! ババ抜きのゲームロジック（バイナリは `main.rs` からこのクレートを利用する）。

pub mod trump;
pub use trump::{Field, Deck, Player};
