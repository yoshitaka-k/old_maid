/// ゲーム参加プレイヤーの状態

use std::time::Duration;
use std::thread;

use super::card::Card;

#[derive(Clone)]
struct CardSet(Vec<Card>);

impl CardSet {
    fn display(&self) {
        let mut msg: String = String::new();

        print!("Player Hand: ");
        for card in self.0.clone() {
            msg = format!("{}{}, ", msg, card.get_name());
        }
        msg = msg.trim().to_string();
        if let Some(m) = msg.strip_suffix(",") {
            println!("{}", m);
        }
    }

    fn add(&mut self, card: Card) {
        self.0.push(card);
    }

    fn remove(&mut self, index: usize) -> Card {
        self.0.remove(index)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn sort(&mut self) {
        self.0.sort_by_key(|c| c.sort_tuple());
    }

    fn get_card(&self, index: usize) -> &Card {
        let cardset = &self.0;
        &cardset[index]
    }

    fn has_jokery(&mut self) -> bool {
        for card in &self.0 {
            if card.get_suit() == "j" {
                return true
            }
        }

        false
    }
}

//////////////////////////////////////////////////

#[derive(Clone)]
struct Status {
    rank: usize,
    joker_turn: usize,
}

impl Status {
    fn new() -> Self {
        Self {
            rank: 0,
            joker_turn: 0,
        }
    }

    fn set_rank(&mut self, rank: usize) {
        self.rank = rank;
    }

    fn get_rank(&self) -> &usize {
        &self.rank
    }

    fn update_joker(&mut self) {
        self.joker_turn = self.joker_turn + 1;
    }

    fn get_joker_turn(&self) -> &usize {
        &self.joker_turn
    }
}

//////////////////////////////////////////////////

#[derive(Clone)]
pub struct Player {
    name: String,
    hand: CardSet,
    discard: Vec<Card>,
    status: Status,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hand: CardSet(vec![]),
            discard: vec![],
            status: Status::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_hand(&mut self, card: Card) {
        self.hand.add(card);
    }

    pub fn remove_hand(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }

    pub fn hand_len(&self) -> usize {
        self.hand.len()
    }

    pub fn hand_is_empty(&mut self) -> bool {
        self.hand.len() == 0
    }

    pub fn set_rank(&mut self, rank: usize) {
        self.status.set_rank(rank);
    }

    pub fn get_rank(&self) -> &usize {
        self.status.get_rank()
    }

    pub fn update_status_joker(&mut self) {
        if self.hand.has_jokery() {
            self.status.update_joker();
        }
    }

    pub fn joker_turn(&self) -> &usize {
        self.status.get_joker_turn()
    }

    pub fn sort_hand(&mut self) {
        self.hand.sort()
    }

    pub fn display_hand(&self) {
        self.hand.display();
    }

    /// ログ用（そのプレイヤーが捨てたカードのコピー）。
    pub fn discard_log(&self) -> &[Card] {
        &self.discard
    }

    /// 手札のうち **同じ数字（rank）** のペアを 1 組捨てる。
    /// 実体は `self.discard` に捨て置きに送り、`field` 側のはログ用のコピーのみ積む。
    pub fn try_discard_pair_same_rank(&mut self) -> Vec<Card> {
        let mut discards = Vec::new();
        let mut pair: Option<(usize, usize)> = None;

        'search: for i in 0..self.hand.len() {
            for j in (i + 1)..self.hand.len() {
                if self.hand.get_card(i).get_rank() == self.hand.get_card(j).get_rank() {
                    pair = Some((i, j));
                    break 'search;
                }
            }
        }

        let Some((i, j)) = pair else {
            return discards;
        };

        // 後ろのインデックスから先に除く（ずれ防止）
        let second = self.hand.remove(j);
        let first = self.hand.remove(i);

        discards.push(first.clone());
        discards.push(second.clone());

        self.discard.push(first);
        self.discard.push(second);

        // 早すぎるから1ペア100ms
        thread::sleep(Duration::from_millis(100));

        discards
    }

    /// ペアが無くなるまで繰り返し捨てる。
    pub fn discard_all_pairs_same_rank(&mut self) -> Vec<Card> {
        let mut total_discards = Vec::new();

        loop {
            let mut pair = self.try_discard_pair_same_rank();
            thread::sleep(Duration::from_millis(100));

            if pair.is_empty() {
                break;
            }

            total_discards.append(&mut pair);
        }

        total_discards
    }
}
