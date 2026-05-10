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
    // 山札の切り方
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        hindu_shuffle(cards, &HinduParams::beginner());
        double_cut(cards);
    }

    // 手札を並び替え
    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();
    }

    // 相手のカードを引く場所
    fn choose_card(&self, len: usize) -> usize {
        if len < 2 {
            return 0
        }
        return 0
    }
}
