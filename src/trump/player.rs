use super::card::Card;
use super::field::Field;

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
    discard: Vec<Card>,
    rank: usize,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hand: vec![],
            discard: vec![],
            rank: 0,
        }
    }

    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }

    pub fn set_rank(&mut self, rank: usize) {
        self.rank = rank;
    }

    pub fn get_rank(&mut self) -> usize {
        self.rank
    }

    pub fn hand_in(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn hand_out(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }

    pub fn get_hand_len(&self) -> usize {
        self.hand.len()
    }

    pub fn hand_is_empty(&self) -> bool {
        self.hand.len() == 0
    }

    pub fn sort_hand(&mut self) {
        self.hand.sort_by_key(|c| c.sort_tuple());
    }

    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }

    pub fn get_hand_cards(&self) -> Vec<String> {
        let mut hands: Vec<String> = Vec::new();
        for _hand in &self.hand {
            hands.push(format!("{}{}", _hand.get_suit(), _hand.get_rank()));
        }
        hands
    }

    /// ログ用（そのプレイヤーが捨てたカードのコピー）。
    pub fn discard_log(&self) -> &[Card] {
        &self.discard
    }

    /// 手札のうち **同じ数字（rank）** のペアを 1 組捨てる。
    /// 実体は `deck` 側の捨て置きに送り、`self.discard` にはログ用のコピーのみ積む。
    pub fn try_discard_pair_same_rank(&mut self, field: &mut Field) -> bool {
        let mut pair: Option<(usize, usize)> = None;
        'search: for i in 0..self.hand.len() {
            for j in (i + 1)..self.hand.len() {
                if self.hand[i].get_rank() == self.hand[j].get_rank() {
                    pair = Some((i, j));
                    break 'search;
                }
            }
        }
        let Some((i, j)) = pair else {
            return false;
        };
        // 後ろのインデックスから先に除く（ずれ防止）
        let second = self.hand.remove(j);
        let first = self.hand.remove(i);

        self.discard.push(first.clone());
        self.discard.push(second.clone());
        field.record_discards([first, second]);

        true
    }

    /// ペアが無くなるまで繰り返し捨てる。
    pub fn discard_all_pairs_same_rank(&mut self, field: &mut Field) {
        while self.try_discard_pair_same_rank(field) {}
    }
}
