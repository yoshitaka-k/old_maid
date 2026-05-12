use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::organize_hand::{
    joker_in_center,
};
use crate::logic::shuffle::{
    deal_shuffle,
    double_cut,
    DealParams,
};
use crate::utils::{rand_range};
use crate::{Card, Player};

//////////////////////////////////////////////////

/// 強さ指定なし
pub struct NoneStrategy;
impl CpuStrategy for NoneStrategy {
    /// 山札の切り方
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        deal_shuffle(cards, &DealParams::default());
        double_cut(cards);
    }

    /// 手札を並び替え
    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();
        let hand = player.get_hand();

        joker_in_center(hand);
    }

    /// 相手のカードを引く場所
    fn choose_card(&self, len: usize) -> usize {
        if len < 1 {
            return 0
        }
        rand_range(0..len)
    }
}
