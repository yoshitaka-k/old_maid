use rand::prelude::SliceRandom;
use crate::cli::console::{read_usize_line, error};

use crate::logic::organize_hand::{
    joker_in_history_taken,
};
use crate::logic::shuffle::{
    double_cut,
    hindu_shuffle,
    riffle_shuffle,
    deal_shuffle,
    HinduParams,
    RiffleParams,
    DealParams,
};

use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 人処理の管理
pub struct Human();
impl Human {
    /// 山札切る処理
    pub fn deck_shuffle(cards: &mut Vec<Card>) {
        hindu_shuffle(cards, &HinduParams::default());
        riffle_shuffle(cards, &RiffleParams::default());
        deal_shuffle(cards, &DealParams::default());
        double_cut(cards);
    }

    /// 手札の並び替え
    pub fn organize_hand(player: &mut Player) {
        let history_token = player.get_history_token_frequency();

        let hand = player.get_hand();
        hand.shuffle(&mut rand::thread_rng());

        joker_in_history_taken(hand, history_token);
    }

    /// 相手の手札から左から何番目を選択する
    pub fn choose_card(players: &Vec<Player>, target_player_idx: usize) -> usize {
        let max_idx = players[target_player_idx].hand_len().saturating_sub(1);

        loop {
            match read_usize_line(&format!(
                        "Draw a card from {} (index from the left 0-{}, Default 0): ",
                        players[target_player_idx].get_name(),
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
