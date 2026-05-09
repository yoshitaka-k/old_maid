pub mod cli;
pub mod game;
pub mod logic;
pub mod trump;
pub mod utils;
pub use cli::console::{error, read_usize_line};
pub use trump::player::PlayerType;
pub use trump::{Card, Deck, Field, GameMode, Player};
pub use utils::rand_range;