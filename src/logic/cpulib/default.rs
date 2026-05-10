use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::cpulib::shuffle::{
    hindu_shuffle,
    double_cut,
    HinduParams,
};
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ指定なし
pub struct NoneStrategy;
impl CpuStrategy for NoneStrategy {
    /// 山札の切り方
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        hindu_shuffle(cards, &HinduParams::beginner());
        double_cut(cards);
    }

    /// 手札を並び替え
    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();

        let hand = player.get_hand();
        hand.sort_by_key(|c| c.sort_tuple());

        if let Some(index) = hand.iter().position(|c| c.is_joker()) {
            let card = hand.remove(index);
            player.add_hand_to_index(0, card);
        }
    }

    /// 相手のカードを引く場所
    fn choose_card(&self, len: usize) -> usize {
        if len < 2 {
            return 0
        }
        return 0
    }
}
