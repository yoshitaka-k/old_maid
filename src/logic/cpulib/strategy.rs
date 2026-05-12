use crate::{Card, Player};

//////////////////////////////////////////////////

/// CPUの強さ
pub trait CpuStrategy {
    /// 山札の切り方
    fn deck_shuffle(&self, deck: &mut Vec<Card>);

    /// 自分の手札を並び替え
    fn organize_hand(&self, player: &mut Player);

    /// 相手のカードを引く場所
    fn choose_card(&self, target_hand_len: usize) -> usize;
}
