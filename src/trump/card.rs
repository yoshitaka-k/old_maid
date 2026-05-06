#[derive(Clone)]
pub struct Card {
    suit: String,
    rank: String,
}

impl Card {
    pub fn new(suit: String, rank: String) -> Self {
        Self { suit, rank }
    }

    pub fn get_suit(&self) -> &String {
        &self.suit
    }

    pub fn get_rank(&self) -> &String {
        &self.rank
    }

    pub fn get_name(&self) -> String {
        let suit = match self.get_suit().as_str() {
            "h" => "♥",
            "d" => "♦",
            "c" => "♣",
            "s" => "♠",
            "j" => "J",
            &_ => todo!(),
        };

        if self.get_rank() == "0" {
            suit.to_string()
        } else {
            format!("{}{}", suit, self.get_rank())
        }
    }

    /// 手札表示用の並び: スート（h → d → c → s → j）、同スート内はランクの数値順。
    pub(crate) fn sort_tuple(&self) -> (u8, u16) {
        let suit = match self.suit.as_str() {
            "h" => 0,
            "d" => 1,
            "c" => 2,
            "s" => 3,
            "j" => 4,
            _ => 5,
        };
        let rank = self.rank.parse::<u16>().unwrap_or(u16::MAX);
        (suit, rank)
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}