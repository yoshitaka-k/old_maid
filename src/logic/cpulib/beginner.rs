use rand::prelude::SliceRandom;

use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::organize_hand::{
    joker_in_first
};
use crate::logic::shuffle::{
    hindu_shuffle,
    HinduParams,
};
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// 強さ初心者
pub struct BeginnerStrategy;
impl CpuStrategy for BeginnerStrategy {
    /// 山札の切り方
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        hindu_shuffle(cards, &HinduParams::beginner());
    }

    /// 手札を並び替え
    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();
        let hand = player.get_hand();

        joker_in_first(hand);
    }

    /// 相手のカードを引く場所
    fn choose_card(&self, len: usize) -> usize {
        if len < 1 {
            return 0
        }
        [0, (len-1)].choose(&mut rand::thread_rng()).unwrap().clone()
    }
}
