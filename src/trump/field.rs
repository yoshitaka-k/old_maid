use super::card::Card;

pub struct Field {
    rank: Vec<String>,
    discard: Vec<Card>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            rank: vec![],
            discard: vec![],
        }
    }

    pub fn set_rank_player_name(&mut self, name: String) {
        self.rank.push(name);
    }

    pub fn get_rank(&mut self) -> Vec<String> {
        self.rank.clone()
    }

    /// 全プレイヤーが捨てたカードを集約（山札用 `discard`）。挿入順＝捨てた時系列のまま積まれる。
    pub fn record_discards(&mut self, cards: impl IntoIterator<Item = Card>) {
        self.discard.extend(cards);
    }

    /// 全員分の捨て札をまとめて参照（誰の捨てかは区別しない一列）。
    pub fn all_discards(&self) -> Vec<String> {
        let mut discards: Vec<String> = Vec::new();
        for _discard in &self.discard {
            discards.push(format!("{}{}", _discard.get_suit(), _discard.get_rank()));
        }
        discards
    }
}