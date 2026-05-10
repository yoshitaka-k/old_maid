/// 人処理
pub mod human;
pub use human::Human;

/// CPU管理処理
pub mod cpu;
pub use cpu::CpuLevelGroup;
pub use cpu::CpuLevel;
pub use cpu::Cpu;

/// 山札の切り方方式
pub mod shuffle;

/// CPUの強さ処理
mod cpulib;
