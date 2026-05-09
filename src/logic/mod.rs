/// 人処理
pub mod human;
pub use human::Human;

/// CPU管理処理
pub mod cpu;
pub use cpu::CpuLevelGroup;
pub use cpu::CpuLevel;
pub use cpu::Cpu;

/// CPUの強さ処理
mod cpulib;
