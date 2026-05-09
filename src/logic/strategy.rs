use rand::prelude::SliceRandom;

use crate::rand_range;
use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// CPUの強さ処理
pub(super) trait CpuStrategy {
    // 山札の切り方ロジック
    fn deck_shuffle(&self, deck: &mut Vec<Card>);

    // 自分の手札を並び替える時のロジック
    fn organize_hand(&self, player: &mut Player);

    // 他のカードを引く時のロジック
    fn choose_card(&self, target_hand_len: usize) -> usize;
}

//////////////////////////////////////////////////

pub(super) struct NoneStrategy;
impl CpuStrategy for NoneStrategy {
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        cards.shuffle(&mut rand::thread_rng());
    }

    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();
    }

    fn choose_card(&self, len: usize) -> usize {
        if len > 0 {
            return 0
        }
        len
    }
}

//////////////////////////////////////////////////

pub(super) struct RandomStrategy;
impl CpuStrategy for RandomStrategy {
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        cards.shuffle(&mut rand::thread_rng());
    }

    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();
    }

    fn choose_card(&self, len: usize) -> usize {
        rand_range(0..len)
    }
}

//////////////////////////////////////////////////

pub(super) struct BeginnerStrategy;
impl CpuStrategy for BeginnerStrategy {
    fn deck_shuffle(&self, cards: &mut Vec<Card>) {
        cards.shuffle(&mut rand::thread_rng());
    }

    fn organize_hand(&self, player: &mut Player) {
        player.sort_hand();
    }

    fn choose_card(&self, len: usize) -> usize {
        if len > 0 {
            return 0
        }
        len
    }
}
