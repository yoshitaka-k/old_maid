/// ターミナルに表示するもの
pub mod cli;
/// ゲーム進行の処理
pub mod game;
/// 人・CPUの処理
pub mod logic;
/// ゲームルール、トランプカード、参加プレイヤーの情報
pub mod trump;
/// 簡易にした関数群
pub mod utils;

pub use cli::console::{error, read_usize_line};
pub use trump::player::PlayerType;
pub use trump::{Card, Deck, Field, GameMode, Player};
pub use utils::rand_range;