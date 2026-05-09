use crate::Card;
use crate::Player;

//////////////////////////////////////////////////

/// CPUの強さ
pub trait CpuStrategy {
    // 山札の切り方ロジック
    fn deck_shuffle(&self, deck: &mut Vec<Card>);

    // 自分の手札を並び替える時のロジック
    fn organize_hand(&self, player: &mut Player);

    // 他のカードを引く時のロジック
    fn choose_card(&self, target_hand_len: usize) -> usize;
}
