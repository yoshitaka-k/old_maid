use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::cpulib::shuffle::{
    double_cut,
    hindu_shuffle,
    riffle_shuffle,
    deal_shuffle,
    HinduParams,
    RiffleParams,
    DealParams,
};
use crate::rand_range;
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ熟練者
pub struct VeteranStrategy;
impl CpuStrategy for VeteranStrategy {
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        hindu_shuffle(cards, &HinduParams::veteran());
        riffle_shuffle(cards, &RiffleParams::veteran());
        deal_shuffle(cards, &DealParams::veteran());
        double_cut(cards);
    }

    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();
    }

    fn choose_card(&self, len: usize) -> usize {
        if len < 1 {
            return 0
        }

        rand_range(0..len)
    }
}
