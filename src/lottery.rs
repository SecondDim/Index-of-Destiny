use rand::seq::IteratorRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::twse_client::MiIndex4;

/// 根據 API 資料產生亂數 seed
fn make_seed(data: &[MiIndex4]) -> u64 {
    let mut total: f64 = 0.0;

    let latest_data = data.last().unwrap();

    total += latest_data.Date.parse::<f64>().unwrap_or(0.0);
    total += latest_data.TradeValue.parse::<f64>().unwrap_or(0.0);
    total += latest_data.FormosaIndex.parse::<f64>().unwrap_or(0.0);
    total += latest_data.Change.parse::<f64>().unwrap_or(0.0);

    // 雜湊總和
    let mut hasher = DefaultHasher::new();
    total.to_bits().hash(&mut hasher); // f64 先轉成 u64 bits
    hasher.finish()
}

/// 產生兩組隨機數字
/// 第一組：1..=49 選 6 個
/// 第二組：1..=38 選 6 個，再從 1..=8 選 1 個 bonus，共 7 個
pub fn generate_lottery(data: &[MiIndex4]) -> (Vec<u8>, Vec<u8>, u8) {
    let seed = make_seed(data);

    // 第一組
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut group1: Vec<u8> = (1u8..=49).choose_multiple(&mut rng, 6);
    group1.sort_unstable();

    // 第二組
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut group2: Vec<u8> = (1u8..=38).choose_multiple(&mut rng, 6);
    group2.sort_unstable();
    let bonus: u8 = (1u8..=8).choose(&mut rng).unwrap();

    (group1, group2, bonus)
}
