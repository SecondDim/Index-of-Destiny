mod lottery;
mod twse_client;

use lottery::generate_lottery;
use std::io::{self, Write};
use twse_client::fetch_mi_index4_json;

#[tokio::main]
async fn main() {
    match fetch_mi_index4_json().await {
        Ok(data) => {
            // 直接拿到 Vec<MiIndex4>，可以當作 JSON 資料操作
            println!("✅ 資料日期：{}", data.last().unwrap().Date);

            // 用 API 回傳的 data 當作亂數種子，產生「第一組」與「第二組+bonus」
            let (group1, group2, bonus) = generate_lottery(&data);

            println!("🎲 第一組 (1–49 選 6)：{:?}", group1);
            println!(
                "🎲 第二組 (1–38 選 6)：{:?}  + Bonus(1–8)：{}",
                group2, bonus
            );
        }
        Err(e) => eprintln!("❌ 抓取失敗：{}", e),
    }

    println!("\n請按 Enter 鍵結束...");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}
