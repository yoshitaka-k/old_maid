use rand::Rng;
use std::thread;
use std::time::Duration;

use crate::Card;

/// ヒンズー回数、落とす枚数の上限の指定
pub struct HinduParams {
    /// ヒンズーを繰り返す回数
    pub iterations: usize,
    /// 山から一度に落とす枚数の下限（この値の乱数〜max）
    pub min_chunk: usize,
    /// 山から一度に落とす枚数の上限（min〜この値の乱数）
    pub max_chunk: usize,
}

impl HinduParams {
    #[allow(dead_code)]
    pub fn beginner() -> Self {
        Self {
            iterations: 5,
            min_chunk: 10,
            max_chunk: 20,
        }
    }

    #[allow(dead_code)]
    pub fn medium() -> Self {
        Self {
            iterations: 10,
            min_chunk: 10,
            max_chunk: 15,
        }
    }

    #[allow(dead_code)]
    pub fn veteran() -> Self {
        Self {
            iterations: 15,
            min_chunk: 5,
            max_chunk: 10,
        }
    }

    #[allow(dead_code)]
    pub fn gambler() -> Self {
        Self {
            iterations: 10,
            min_chunk: 5,
            max_chunk: 20,
        }
    }
}

impl Default for HinduParams {
    fn default() -> Self {
        Self {
            iterations: 10,
            min_chunk: 10,
            max_chunk: 20,
        }
    }
}

/// 山札の切り方（ヒンズー方式）
// 各CPU強さ処理から `hindu_shuffle` に渡す。
pub fn hindu_shuffle(cards: &mut Vec<Card>, params: &HinduParams) {
    let iterations = params.iterations.max(1);
    let min_chunk = params.min_chunk.max(1);
    let max_chunk = params.max_chunk.max(min_chunk);

    for _ in 0..iterations {
        hindu_shuffle_once(cards, min_chunk, max_chunk);

        // 早すぎるから100ms待ち
        thread::sleep(Duration::from_millis(100));
    }
}

/// 山札の切り方（ヒンズー方式）
/// * `min_chunks` - 1度に落とすカードのブレ数（下限）
/// * `max_chunks` - 1度に落とすカードのブレ数（上限）
fn hindu_shuffle_once(cards: &mut Vec<Card>, min_chunk: usize, max_chunk: usize) {
    if cards.len() < 2 {
        return;
    }

    let mut rng = rand::thread_rng();
    let mut mixed = Vec::with_capacity(cards.len());
    let mut left = std::mem::take(cards);

    while !left.is_empty() {
        let n = left.len();
        if n == 1 {
            mixed.extend(left.drain(..));
            break;
        }
        let jitter = (left.len() / 10).max(1);
        let jitter_hi = ((n.saturating_sub(2)) as isize).max(1);

        let min_cut = rng
            .gen_range(-(jitter as isize)..=(jitter as isize))
            .clamp(-(jitter_hi), jitter_hi) as isize;
        let max_cut = rng
            .gen_range(-(jitter as isize)..=(jitter as isize))
            .clamp(-(jitter_hi), jitter_hi) as isize;

        let mut chunk_min = (min_chunk as isize + min_cut).clamp(1, n as isize) as usize;
        let mut chunk_max = (max_chunk as isize + max_cut).clamp(1, n as isize) as usize;
        chunk_min = chunk_min.min(n);
        chunk_max = chunk_max.min(n);
        if chunk_min > chunk_max {
            std::mem::swap(&mut chunk_min, &mut chunk_max);
        }

        let take_n = rng.gen_range(chunk_min..=chunk_max);
        let start = left.len() - take_n;

        mixed.extend(left.drain(start..));

        // 早すぎるから20ms待ち
        thread::sleep(Duration::from_millis(20));
    }

    *cards = mixed;
}
