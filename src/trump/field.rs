use crate::Card;
use crate::Player;

/// 順位情報
struct Ranking(Vec<Player>);

impl Ranking {
    fn add(&mut self, player: Player) {
        self.0.push(player);
    }

    fn get(&self) -> &Vec<Player>{
        &self.0
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

//////////////////////////////////////////////////

/// ゲームフィールド
pub struct Field {
    rank: Ranking,
    discard: Vec<Card>,
    mystery_card: Option<Card>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            rank: Ranking(vec![]),
            discard: vec![],
            mystery_card: None,
        }
    }

    pub fn clear(&mut self) {
        self.rank.clear();
        self.discard.clear();
        self.mystery_card = None;
    }

    pub fn add_rank(&mut self, player: Player) {
        self.rank.add(player);
    }

    pub fn get_rank(&self) -> &Vec<Player> {
        self.rank.get()
    }

    pub fn get_rank_len(&self) -> usize {
        self.rank.len()
    }

    pub fn set_mystery_card(&mut self, mystery_card: Option<Card>) {
        self.mystery_card = mystery_card;
    }

    pub fn get_mystery_card(&self) -> &Option<Card> {
        &self.mystery_card
    }

    pub fn get_mystery_card_name(&self) -> String {
        match &self.mystery_card {
            Some(c) => c.get_name(),
            None => "".to_string(),
        }
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
