use rand::prelude::SliceRandom;

use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::organize_hand::{
    joker_in_last,
};
use crate::logic::shuffle::{
    riffle_shuffle,
    hindu_shuffle,
    RiffleParams,
    HinduParams,
};
use crate::utils::{get_center_position};
use crate::{Card, Player};

//////////////////////////////////////////////////

/// 強さ中くらい
pub struct MediumStrategy;
impl CpuStrategy for MediumStrategy {
    /// 山札の切り方
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        hindu_shuffle(cards, &HinduParams::medium());
        riffle_shuffle(cards, &RiffleParams::medium());
    }

    /// 手札を並び替え
    /// 引かれた箇所が多い位置にジョーカーを持っていく
    fn organize_hand(&self, player: &mut Player) {
        let hand = player.get_hand();
        hand.shuffle(&mut rand::thread_rng());

        joker_in_last(hand);
    }

    /// 相手のカードを引く場所
    fn choose_card(&self, len: usize) -> usize {
        if len < 1 {
            return 0
        }
        get_center_position(len)
    }
}
