use rand::prelude::SliceRandom;

use crate::logic::cpulib::{
    strategy::CpuStrategy,
    default::NoneStrategy,
    random::RandomStrategy,
    beginner::BeginnerStrategy,
    medium::MediumStrategy,
    gambler::GamblerStrategy,
    veteran::VeteranStrategy,
};

use crate::{Card, Player, PlayerType};

//////////////////////////////////////////////////

/// CPU強さグループ
#[derive(Debug, Copy, Clone)]
pub enum CpuLevelGroup {
    None,
    Beginner,
    Medium,
    Veteran,
}

/// CPU強さ
#[derive(Clone)]
pub enum CpuLevel {
    None,
    Beginner,
    Medium,
    Veteran,
    Gambler,
    Random,
}

//////////////////////////////////////////////////

/// CPU処理の管理
pub struct Cpu();
impl Cpu {
    pub fn new() -> Self {
        Self()
    }

    /// CPUの強さグループ
    /// 6分の1でCPUの強さを決める
    fn level_choices(level_group: &CpuLevelGroup) -> [CpuLevel; 6] {
        match level_group {
            CpuLevelGroup::None => [
                CpuLevel::None, CpuLevel::None, CpuLevel::None,
                CpuLevel::None, CpuLevel::None, CpuLevel::None,
            ],
            CpuLevelGroup::Beginner => [
                CpuLevel::Beginner, CpuLevel::Beginner, CpuLevel::Beginner,
                CpuLevel::Medium,   CpuLevel::Medium,
                CpuLevel::Random,
            ],
            CpuLevelGroup::Medium => [
                CpuLevel::Beginner,  CpuLevel::Medium,  CpuLevel::Medium,
                CpuLevel::Medium, CpuLevel::Gambler, CpuLevel::Random,
            ],
            CpuLevelGroup::Veteran => [
                CpuLevel::Medium,  CpuLevel::Veteran,  CpuLevel::Veteran,
                CpuLevel::Veteran, CpuLevel::Gambler, CpuLevel::Random,
            ],
        }
    }

    /// CPUの強さ設定
    /// 強さグループから6分の1でCPUの強さを決める
    pub fn new_level(level_group: &CpuLevelGroup) -> CpuLevel {
        let choices = Self::level_choices(&level_group);
        choices.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    //////////////////////////////////////////////////

    fn _get_strategy(&self, player_type: &PlayerType) -> Box<dyn CpuStrategy> {
        match player_type {
            PlayerType::Human => Box::new(RandomStrategy),
            PlayerType::Cpu(level) => {
                match level {
                    CpuLevel::None => Box::new(NoneStrategy),
                    CpuLevel::Beginner => Box::new(BeginnerStrategy),
                    CpuLevel::Medium => Box::new(MediumStrategy),
                    CpuLevel::Veteran => Box::new(VeteranStrategy),
                    CpuLevel::Gambler => Box::new(GamblerStrategy),
                    CpuLevel::Random => Box::new(RandomStrategy),
                }
            }
        }
    }

    /// 山札の切り方
    pub fn deck_shuffle(&self, player: &Player, cards: &mut Vec<Card>) {
        let player_type = player.get_player_type();
        let strategy = self._get_strategy(player_type);
        strategy.deck_shuffle(cards);
    }

    /// 手札を並び替え
    pub fn organize_hand(&self, player: &mut Player) {
        let player_type = player.get_player_type();

        let strategy = self._get_strategy(player_type);
        strategy.organize_hand(player);
    }

    /// 相手のカードを引く場所
    pub fn choose_card(&self, players: &Vec<Player>, current: usize, target_player_idx: usize) -> usize {
        let player_type = players[current].get_player_type();
        let max_idx = players[target_player_idx].hand_len();

        if max_idx == 0 {
            return 0
        }

        let strategy = self._get_strategy(player_type);
        strategy.choose_card(max_idx)
    }
}
