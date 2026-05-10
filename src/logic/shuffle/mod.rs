mod hindu;
mod riffle;
mod deal;
mod double_cut;

pub use double_cut::double_cut;
pub use hindu::{hindu_shuffle, HinduParams};
pub use riffle::{riffle_shuffle, RiffleParams};
pub use deal::{deal_shuffle, DealParams};
