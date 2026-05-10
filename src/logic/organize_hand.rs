use crate::utils::get_center_position;
use crate::Card;

/// 一番初めの位置にジョーカーを持っていく
pub fn joker_in_first(hand: &mut Vec<Card>) {
    if let Some(index) = hand.iter().position(|c| c.is_joker()) {
        let joker = hand.remove(index);
        hand.insert(0, joker);
    }
}

/// 一番真ん中辺りの位置にジョーカーを持っていく
pub fn joker_in_center(hand: &mut Vec<Card>) {
    if let Some(index) = hand.iter().position(|c| c.is_joker()) {
        let joker = hand.remove(index);
        let len = hand.len();

        // 残り手札が少ない場合は範囲が壊れないよう固定位置に挿入する
        if len <= 1 {
            hand.insert(len, joker);
            return;
        }

        let center = get_center_position(len);

        hand.insert(center, joker);
    }
}

/// 一番後ろの位置にジョーカーを持っていく
pub fn joker_in_last(hand: &mut Vec<Card>) {
    if let Some(index) = hand.iter().position(|c| c.is_joker()) {
        let joker = hand.remove(index);
        hand.insert(hand.len(), joker);
    }
}

/// 引かれる場所が多いところにジョーカーを持っていく
pub fn joker_in_history_taken(hand: &mut Vec<Card>, history_token: Vec<usize>) {
    if let Some(joker_index) = hand.iter().position(|c| c.is_joker()) {
        let insert_index = history_token
            .into_iter()
            .find(|&i| i < hand.len())
            .unwrap_or(0);

        let joker = hand.remove(joker_index);
        hand.insert(insert_index, joker);
    }
}
