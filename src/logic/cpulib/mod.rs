/// CPU強さ毎の判定
pub(super) mod strategy;
/// 強さ指定なし
pub(super) mod default;
/// 強さ乱数任せ
pub(super) mod random;
/// 強さ初心者
pub(super) mod beginner;
/// 強さ中くらい
pub(super) mod medium;
/// 強さ博奕打ち
pub(super) mod gambler;
/// 強さ熟練者
pub(super) mod veteran;

use rand::Rng;
use crate::Card;

/// 山札の切り方
/// リフルシャッフル方式
pub(crate) fn riffle_shuffle(cards: &mut Vec<Card>) {
    if cards.len() < 2 {
        return;
    }

    let mut rng = rand::thread_rng();

    // だいたい真ん中付近で山を2つに切る（少しだけランダム）。
    let base = cards.len() / 2;
    let jitter = (cards.len() / 10).max(1);
    let cut = (base as isize + rng.gen_range(-(jitter as isize)..=(jitter as isize)))
        .clamp(1, cards.len() as isize - 1) as usize;

    let mut right = cards.split_off(cut);
    let mut left = std::mem::take(cards);
    let mut mixed = Vec::with_capacity(left.len() + right.len());

    while !left.is_empty() || !right.is_empty() {
        let take_left = if left.is_empty() {
            false
        } else if right.is_empty() {
            true
        } else {
            let total = left.len() + right.len();
            rng.gen_ratio(left.len() as u32, total as u32)
        };

        let pile = if take_left { &mut left } else { &mut right };
        let take_n = rng.gen_range(1..=pile.len().min(3));
        let start = pile.len() - take_n;
        mixed.extend(pile.drain(start..));
    }

    *cards = mixed;
}