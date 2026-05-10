use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::cpulib::shuffle::{
    riffle_shuffle,
    double_cut,
    RiffleParams,
};
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ中くらい
pub struct MediumStrategy;
impl CpuStrategy for MediumStrategy {
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        riffle_shuffle(cards, &RiffleParams::medium());
        double_cut(cards);
    }

    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();
    }

    fn choose_card(&self, len: usize) -> usize {
        if len < 2 {
            return 0
        }
        return 0
    }
}
