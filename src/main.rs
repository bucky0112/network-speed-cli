use reqwest;
use std::io::Write;
use std::time::{Duration, Instant};
use tokio::task;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // let url = "http://http.speed.hinet.net/test_200m.zip";
    let url = "http://http.speed.hinet.net/test_250m.zip";
    // let url = "http://http.speed.hinet.net/test_1024m.zip";

    // 創建動態更新的任務
    let loading_dots_task = task::spawn(async {
        let mut count = 0;
        loop {
            sleep(Duration::from_secs(1)).await;
            count += 1;
            if count <= 3 {
                print!(".");
            } else {
                // 重置計數器並移動光標回到起始位置，再次開始打印
                count = 0;
                print!("\r開始測試下載速度 ");
            }
            // 強制立即輸出，而不緩存
            std::io::stdout().flush().unwrap();
        }
    });

    let start_time = Instant::now();

    match reqwest::get(url).await {
        Ok(response) => {
            if !response.status().is_success() {
                println!("測試下載時發生錯誤: {:?}", response.status());
                return;
            }

            match response.bytes().await {
                Ok(bytes) => {
                    let duration = start_time.elapsed();
                    let speed_in_mbps =
                        (bytes.len() * 8) as f64 / duration.as_secs_f64() / 1_000_000.0;
                    // 終止動態更新任務並清除留下的 "."
                    loading_dots_task.abort();
                    print!("\r{: <30}", ""); // 使用空格覆蓋整行
                    print!("\r"); // 移動光標回行首

                    println!("網路下載速度: {:.2} Mbps", speed_in_mbps);
                    println!("下載完成，耗時: {:.2} 秒", duration.as_secs_f64());
                }
                Err(e) => {
                    println!("讀取下載資料時發生錯誤: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("下載時發生錯誤: {:?}", e);
        }
    }
}
