/// CPU強さ毎の判定
pub(super) mod strategy;
/// 強さ指定なし
pub(super) mod default;
/// 強さ乱数任せ
pub(super) mod random;
/// 強さ初心者
pub(super) mod beginner;
/// 強さ中くらい
pub(super) mod medium;
/// 強さ博奕打ち
pub(super) mod gambler;
/// 強さ熟練者
pub(super) mod veteran;
