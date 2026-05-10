use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::cpulib::shuffle::{
    deal_shuffle,
    double_cut,
    DealParams,
};
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ指定なし
pub struct NoneStrategy;
impl CpuStrategy for NoneStrategy {
    /// 山札の切り方
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        deal_shuffle(cards, &DealParams::beginner());
        double_cut(cards);
    }

    /// 手札を並び替え
    fn organize_hand(&self, player: &mut Player) {
        let history_token = player.get_history_token_frequency();
        let hand = player.get_hand();
        hand.sort_by_key(|c| c.sort_tuple());

        if let Some(joker_index) = hand.iter().position(|c| c.is_joker()) {
            let insert_index = history_token
                .into_iter()
                .find(|&i| i < hand.len())
                .unwrap_or(0);

            let joker = hand.remove(joker_index);
            hand.insert(insert_index, joker);
        }
    }

    /// 相手のカードを引く場所
    fn choose_card(&self, len: usize) -> usize {
        if len < 1 {
            return 0
        }
        return 0
    }
}
