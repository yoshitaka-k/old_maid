use rand::Rng;
use std::thread;
use std::time::Duration;

use crate::utils::get_center_position;
use crate::Card;

/// リフル回数・1回あたりに落とす枚数の上限の指定
// 各CPU強さ処理から `riffle_shuffle` に渡す。
pub struct RiffleParams {
    /// リフルを繰り返す回数
    pub iterations: usize,
    /// 山から一度に落とす枚数の上限（1〜この値の乱数）
    pub max_chunk: usize,
}

impl RiffleParams {
    #[allow(dead_code)]
    pub fn beginner() -> Self {
        Self {
            iterations: 2,
            max_chunk: 5,
        }
    }

    #[allow(dead_code)]
    pub fn medium() -> Self {
        Self {
            iterations: 4,
            max_chunk: 2,
        }
    }

    #[allow(dead_code)]
    pub fn veteran() -> Self {
        Self {
            iterations: 8,
            max_chunk: 2,
        }
    }

    #[allow(dead_code)]
    pub fn gambler() -> Self {
        Self {
            iterations: 8,
            max_chunk: 2,
        }
    }
}

impl Default for RiffleParams {
    fn default() -> Self {
        Self {
            iterations: 2,
            max_chunk: 3,
        }
    }
}

/// 山札の切り方（リフル式）
// * `params`  - 強さごとの「何回やるか・何枚ずつ落とすか」の指定
pub fn riffle_shuffle(cards: &mut Vec<Card>, params: &RiffleParams) {
    let iterations = params.iterations.max(1);
    let max_chunk = params.max_chunk.max(1);

    for _ in 0..iterations {
        riffle_shuffle_once(cards, max_chunk);

        // 早すぎるから100ms待ち
        thread::sleep(Duration::from_millis(100));
    }
}

/// 山札の切り方（リフル式）
/// * `max_chunks` - 1度に落とすカードのブレ数
fn riffle_shuffle_once(cards: &mut Vec<Card>, max_chunk: usize) {
    if cards.len() < 2 {
        return;
    }

    let mut rng = rand::thread_rng();

    // だいたい真ん中付近で山を2つに切る（少しだけランダム）。
    let cut = get_center_position(cards.len());

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
        let chunk_cap = pile.len().min(max_chunk);
        let take_n = rng.gen_range(1..=chunk_cap);
        let start = pile.len() - take_n;
        mixed.extend(pile.drain(start..));

        // 早すぎるから20ms待ち
        thread::sleep(Duration::from_millis(20));
    }

    *cards = mixed;
}
