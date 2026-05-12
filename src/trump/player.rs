use std::thread;
use std::time::Duration;
use std::collections::HashMap;

use crate::logic::CpuLevel;
use crate::Card;

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

        print!("  Player Hand: ");
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

    fn add_to_index(&mut self, index: usize, card: Card) {
        self.0.insert(index, card);
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

    fn has_joker(&self) -> bool {
        self.0.iter().any(Card::is_joker)
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

//////////////////////////////////////////////////

/// 順位情報、ジョーカー保持ターン数
#[derive(Clone)]
struct Status {
    rank: usize,
    point: usize,
    joker_turn: usize,
}

impl Status {
    fn new() -> Self {
        Self {
            rank: 0,
            point: 0,
            joker_turn: 0,
        }
    }

    fn set_rank(&mut self, rank: usize) {
        self.rank = rank;
    }

    fn get_rank(&self) -> usize {
        self.rank
    }

    fn update_point(&mut self, point: usize) {
        self.point = self.point + point
    }

    fn get_point(&self) -> usize {
        self.point
    }

    fn update_joker_turn(&mut self) {
        self.joker_turn = self.joker_turn + 1;
    }

    fn get_joker_turn(&self) -> usize {
        self.joker_turn
    }

    fn clear(&mut self) {
        self.rank = 0;
        self.joker_turn = 0;
    }
}

//////////////////////////////////////////////////

/// プレイヤーに関する履歴情報
#[derive(Clone)]
struct History {
    discard: Vec<Card>,
    rank: Vec<usize>,
    point: Vec<usize>,
    joker_turn: Vec<usize>,
    choose_index: Vec<usize>,
    taken_index: Vec<usize>,
}

impl History {
    pub fn new() -> Self{
        Self {
            discard: vec![],
            rank: vec![],
            point: vec![],
            joker_turn: vec![],
            choose_index: vec![],
            taken_index: vec![],
        }
    }

    fn add_rank(&mut self, rank: usize) {
        self.rank.push(rank);
    }

    fn get_rank(&self) -> &Vec<usize> {
        &self.rank
    }

    fn add_point(&mut self, point: usize) {
        self.point.push(point);
    }

    fn get_point(&self) -> &Vec<usize> {
        &self.point
    }

    fn add_joker_turn(&mut self, turn: usize) {
        self.joker_turn.push(turn);
    }

    fn get_joker_turn(&self) -> &Vec<usize> {
        &self.joker_turn
    }

    fn add_choose_index(&mut self, index: usize) {
        self.choose_index.push(index);
    }

    fn add_taken_index(&mut self, index: usize) {
        self.taken_index.push(index);
    }

    fn get_taken_index(&self) -> &Vec<usize> {
        &self.taken_index
    }

    /// 引かれた数値が多い順に出力
    fn values_by_descending_frequency(&self) -> Vec<usize> {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        for &idx in &self.taken_index {
            *counts.entry(idx).or_insert(0) += 1;
        }
        let mut pairs: Vec<(usize, usize)> = counts.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        pairs.into_iter().map(|(k, _)| k).collect()
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

    pub fn add_hand_to_index(&mut self, index: usize, card: Card) {
        self.hand.add_to_index(index, card);
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

    pub fn hand_clear(&mut self) {
        self.hand.clear();
    }

    /// 相手の手札の左から何番目取った履歴
    pub fn add_history_choose_index(&mut self, index: usize) {
        self.history.add_choose_index(index);
    }

    /// 相手に手札の左から何番目取られた履歴
    pub fn add_history_taken_index(&mut self, index: usize) {
        self.history.add_taken_index(index);
    }

    /// 履歴に保存している引かれた場所リストを返す
    pub fn get_history_taken(&self) -> &Vec<usize> {
        self.history.get_taken_index()
    }

    /// 引かれた数値が多い順に出力
    pub fn get_history_token_frequency(&self) -> Vec<usize> {
        self.history.values_by_descending_frequency()
    }

    pub fn get_history_rank(&self) -> &Vec<usize> {
        self.history.get_rank()
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

    /// 順位ポイント加算、獲得したポイント保存
    pub fn update_point(&mut self, point: usize) {
        self.status.update_point(point);
        self.history.add_point(point);
    }

    pub fn get_point(&self) -> usize {
        self.status.get_point()
    }

    pub fn get_history_point(&self) -> &Vec<usize> {
        self.history.get_point()
    }

    /// 保持ジョーカーターン数を履歴に追加
    pub fn update_history_joker_turn(&mut self) {
        let turn = self.status.get_joker_turn();
        self.history.add_joker_turn(turn);
    }

    /// 保持ジョーカーターン数の取得
    pub fn get_history_joker_turn(&self) -> &Vec<usize> {
        self.history.get_joker_turn()
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

    pub fn status_clear(&mut self) {
        self.status.clear();
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

    pub fn player_type_name(&self) -> String {
        let type_name = match self.get_player_type() {
            PlayerType::Human => "Human",
            PlayerType::Cpu(lv) => match lv {
                CpuLevel::None => "None",
                CpuLevel::Random => "Random",
                CpuLevel::Beginner => "Beginner",
                CpuLevel::Medium => "Medium",
                CpuLevel::Gambler => "Gambler",
                CpuLevel::Veteran => "Veteran",
            },
        };

        type_name.to_string()
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
