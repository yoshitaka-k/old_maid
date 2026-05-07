/// お手軽？関数群

use rand::{Rng, thread_rng};
use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;

//////////////////////////////////////////////////

pub fn rand<T, R>(range: R) -> T
    where
        R: SampleRange<T>, T: SampleUniform {
    thread_rng().gen_range(range)
}

pub fn dice_role() -> usize {
    let dice1 = rand(1..=6);
    let dice2 = rand(1..=6);

    dice1 + dice2
}
