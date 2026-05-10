use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::cpulib::shuffle::{
    riffle_shuffle,
    hindu_shuffle,
    RiffleParams,
    HinduParams,
};
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ中くらい
pub struct MediumStrategy;
impl CpuStrategy for MediumStrategy {
    /// 山札の切り方
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        hindu_shuffle(cards, &HinduParams::medium());
        riffle_shuffle(cards, &RiffleParams::medium());
    }

    /// 手札を並び替え
    /// 引かれた箇所が多い位置にジョーカーを持っていく
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
