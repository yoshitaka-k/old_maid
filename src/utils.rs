/// お手軽？関数群

use rand::{ Rng, thread_rng };
use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;

//////////////////////////////////////////////////

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
