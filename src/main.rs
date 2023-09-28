use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    // let url = "http://http.speed.hinet.net/test_200m.zip";
    let url = "http://http.speed.hinet.net/test_250m.zip";
    // let url = "http://http.speed.hinet.net/test_1024m.zip";

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} 開始測試下載速度 {spinner:.green}")
            .unwrap_or_else(|e| panic!("{}", e)),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    let start_time = Instant::now();

    match reqwest::get(url).await {
        Ok(response) => {
            if !response.status().is_success() {
                println!("測試下載時發生錯誤: {:?}", response.status());
                spinner.finish_and_clear();
                return;
            }

            match response.bytes().await {
                Ok(bytes) => {
                    let duration = start_time.elapsed();
                    let speed_in_mbps =
                        (bytes.len() * 8) as f64 / duration.as_secs_f64() / 1_000_000.0;

                    spinner.finish_and_clear();

                    println!("網路下載速度: {:.2} Mbps", speed_in_mbps);
                    println!("下載完成，耗時: {:.2} 秒", duration.as_secs_f64());
                }
                Err(e) => {
                    spinner.finish_and_clear();
                    println!("讀取下載資料時發生錯誤: {:?}", e);
                }
            }
        }
        Err(e) => {
            spinner.finish_and_clear();
            println!("下載時發生錯誤: {:?}", e);
        }
    }
}
