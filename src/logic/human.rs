use rand::prelude::SliceRandom;

use crate::Card;
use crate::Player;

use crate::read_usize_line;
use crate::error;

//////////////////////////////////////////////////

/// 人処理の管理
pub struct Human();
impl Human {
    /// 人だった場合の山札切る処理
    pub fn deck_shuffle(deck: &mut Vec<Card>) {
        deck.shuffle(&mut rand::thread_rng());
    }

    /// 手札の並び替え
    pub fn organize_hand(player: &mut Player) {
        player.sort_hand();
    }

    /// 相手の手札から左から何番目を選択する
    pub fn input_choose_index(players: &Vec<Player>, target_player_idx: &usize) -> usize {
        let max_idx = players[*target_player_idx].hand_len().saturating_sub(1);

        loop {
            match read_usize_line(&format!(
                        "Draw a card from {} (index from the left 0-{}, Default 0): ",
                        players[*target_player_idx].get_name(),
                        max_idx
                    ), 0) {
                Ok(num) if (0..=max_idx).contains(&num) => {
                    break num;
                },
                Ok(_) => error(&format!("The input is not a number 0-{}.", max_idx)),
                Err(_) => error("The input is not a number."),
            }
        }
    }
}