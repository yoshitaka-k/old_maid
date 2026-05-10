use rand::prelude::SliceRandom;

use crate::logic::cpulib::strategy::CpuStrategy;
use crate::rand_range;
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ乱数任せ
pub struct RandomStrategy;
impl CpuStrategy for RandomStrategy {
    /// 山札の切り方
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        cards.shuffle(&mut rand::thread_rng());
    }

    /// 手札を並び替え
    fn organize_hand(&self, player: &mut Player) {
        let hand = player.get_hand();
        hand.shuffle(&mut rand::thread_rng());
    }

    /// 相手のカードを引く場所
    fn choose_card(&self, len: usize) -> usize {
        if len < 1 {
            return 0
        }

        rand_range(0..len)
    }
}
