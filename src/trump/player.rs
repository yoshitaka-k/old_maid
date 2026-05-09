use std::time::Duration;
use std::thread;

use crate::Card;
use crate::logic::CpuLevel;

//////////////////////////////////////////////////

/// プレイヤー、CPU判断
#[derive(Clone)]
pub enum PlayerType {
    Human,
    Cpu(CpuLevel),
}

//////////////////////////////////////////////////

/// プレイヤー毎の手札構造体
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

    fn get(&mut self) -> &mut Vec<Card> {
        &mut self.0
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

    fn has_joker(&mut self) -> bool {
        for card in &self.0 {
            if card.get_suit() == "j" {
                return true
            }
        }

        false
    }
}

//////////////////////////////////////////////////

/// 順位情報、ジョーカー保持ターン数
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

    fn get_rank(&self) -> usize {
        self.rank
    }

    fn update_joker_turn(&mut self) {
        self.joker_turn = self.joker_turn + 1;
    }

    fn get_joker_turn(&self) -> usize {
        self.joker_turn
    }
}

//////////////////////////////////////////////////

/// プレイヤーに関する履歴情報
#[derive(Clone)]
struct History {
    discard: Vec<Card>,
    rank: Vec<usize>,
    choose_index: Vec<usize>,
}

impl History {
    pub fn new() -> Self{
        Self {
            discard: vec![],
            rank: vec![],
            choose_index: vec![],
        }
    }

    fn add_rank(&mut self, rank: usize) {
        self.rank.push(rank);
    }

    fn add_choose_index(&mut self, index: usize) {
        self.choose_index.push(index);
    }
}

//////////////////////////////////////////////////

/// プレイヤー情報
#[derive(Clone)]
pub struct Player {
    name: String,
    player_type: PlayerType,
    hand: CardSet,
    status: Status,
    history: History,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            player_type: PlayerType::Human,
            hand: CardSet(vec![]),
            status: Status::new(),
            history: History::new(),
        }
    }

    /// プレイヤー名取得
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// 手札にカードを1枚追加
    pub fn add_hand(&mut self, card: Card) {
        self.hand.add(card);
    }

    /// 手札のカードを1枚選択して取り出し削除
    pub fn remove_hand(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }

    pub fn get_hand(&mut self) -> &mut Vec<Card> {
        self.hand.get()
    }

    /// 手札のカードの枚数
    pub fn hand_len(&self) -> usize {
        self.hand.len()
    }

    /// 手札並び替え
    pub fn sort_hand(&mut self) {
        self.hand.sort()
    }

    /// 手札整理（デフォルト）
    pub fn display_hand(&self) {
        self.hand.display();
    }

    /// 手札のカードがない？
    pub fn hand_is_empty(&mut self) -> bool {
        self.hand.len() == 0
    }

    /// 相手の手札の左から何番目取った履歴
    pub fn add_history_choose_index(&mut self, index: usize) {
        self.history.add_choose_index(index);
    }

    /// 上がり順の保持、上がり順履歴に保持
    pub fn set_rank(&mut self, rank: usize) {
        self.status.set_rank(rank);
        self.history.add_rank(rank);
    }

    /// 上がり順の取得
    pub fn get_rank(&self) -> usize {
        self.status.get_rank()
    }

    /// ジョーカーを持っている（持っていた）ターン数
    pub fn update_status_joker_turn(&mut self) {
        if self.hand.has_joker() {
            self.status.update_joker_turn();
        }
    }

    /// 保持ジョーカーターン数の取得
    pub fn get_joker_turn(&self) -> usize {
        self.status.get_joker_turn()
    }

    /// プレイヤーが手動？
    pub fn has_human(&self) -> bool {
        match &self.player_type {
            PlayerType::Human => true,
            PlayerType::Cpu(_) => false,
        }
    }

    pub fn set_player_type(&mut self, level: PlayerType) {
        self.player_type = level;
    }

    pub fn get_player_type(&self) -> &PlayerType {
        &self.player_type
    }

    /// ログ用（そのプレイヤーが捨てたカード）
    pub fn discard_log(&self) -> &[Card] {
        &self.history.discard
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

        self.history.discard.push(first);
        self.history.discard.push(second);

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
