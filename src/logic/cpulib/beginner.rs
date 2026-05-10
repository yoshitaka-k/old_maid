use crate::logic::cpulib::strategy::CpuStrategy;
use crate::logic::cpulib::shuffle::{
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
    /// 一番初めの位置にジョーカーを持っていく
    fn organize_hand(&self, player: &mut Player) {
        let hand = player.get_hand();
        hand.sort_by_key(|c| c.sort_tuple());

        if let Some(index) = hand.iter().position(|c| c.is_joker()) {
            let joker = hand.remove(index);
            hand.insert(0, joker);
        }
    }

    /// 相手のカードを引く場所
    fn choose_card(&self, len: usize) -> usize {
        if len < 1 {
            return 0
        }
        return 0
    }
}
