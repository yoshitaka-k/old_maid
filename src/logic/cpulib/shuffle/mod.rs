mod hindu;
mod riffle;
mod double_cut;

pub(super) use double_cut::double_cut;
pub(super) use hindu::{hindu_shuffle, HinduParams};
pub(super) use riffle::{riffle_shuffle, RiffleParams};
