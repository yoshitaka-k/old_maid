use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::cpulib::shuffle::{
    riffle_shuffle,
    RiffleParams,
};
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ初心者
pub struct BeginnerStrategy;
impl CpuStrategy for BeginnerStrategy {
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        riffle_shuffle(cards, &RiffleParams::beginner());
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
