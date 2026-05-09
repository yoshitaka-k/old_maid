use rand::prelude::SliceRandom;

use crate::logic::cpulib::strategy::CpuStrategy;
use crate::rand_range;
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ乱数任せ
pub struct RandomStrategy;
impl CpuStrategy for RandomStrategy {
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        cards.shuffle(&mut rand::thread_rng());
    }

    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();
    }

    fn choose_card(&self, len: usize) -> usize {
        if len == 0 {
            return 0
        }

        rand_range(0..len)
    }
}

