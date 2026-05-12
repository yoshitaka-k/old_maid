use rand::{ Rng, thread_rng };
use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;

//////////////////////////////////////////////////

/// 真ん中あたりの位置を取得（少しだけランダム）
pub fn get_center_position(cards_len: usize) -> usize {
    if cards_len == 0 {
        return 0;
    }

    let mut rng = rand::thread_rng();
    let base = cards_len / 2;
    let jitter = (cards_len / 10).max(1);
    (base as isize + rng.gen_range(-(jitter as isize)..=(jitter as isize)))
        .clamp(0, cards_len as isize - 1) as usize
}

/// 頭文字だけ大文字にする簡単な関数っ
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// 範囲指定で乱数の生成、長いから簡略化
pub fn rand_range<T, R>(range: R) -> T
    where
        R: SampleRange<T>, T: SampleUniform {
    thread_rng().gen_range(range)
}

/// 麻雀のサイ振り的な処理
pub fn dice_role() -> usize {
    let dice1 = rand_range(1..=6);
    let dice2 = rand_range(1..=6);

    dice1 + dice2
}
