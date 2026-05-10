use crate::logic::cpulib::{
    riffle_shuffle,
    double_cut,
    RiffleParams,
    strategy::CpuStrategy,
};
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ熟練者
pub struct VeteranStrategy;
impl CpuStrategy for VeteranStrategy {
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        riffle_shuffle(cards, &RiffleParams::veteran());
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
