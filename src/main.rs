use indicatif::{ProgressBar, ProgressStyle};
use reqwest;
use std::time::{Duration, Instant};

async fn download_speed(url: &str) -> Result<(f64, f64), String> {
    let start_time = Instant::now();

    match reqwest::get(url).await {
        Ok(response) => {
            if !response.status().is_success() {
                return Err(format!("HTTP Error: {:?}", response.status()));
            }
            match response.bytes().await {
                Ok(bytes) => {
                    let duration = start_time.elapsed();
                    let speed_in_mbps =
                        (bytes.len() * 8) as f64 / duration.as_secs_f64() / 1_000_000.0;
                    Ok((speed_in_mbps, duration.as_secs_f64()))
                }
                Err(e) => Err(format!("Error reading bytes: {:?}", e)),
            }
        }
        Err(e) => Err(format!("Download error: {:?}", e)),
    }
}

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

    match download_speed(url).await {
        Ok((speed, duration)) => {
            spinner.finish_and_clear();
            println!("網路下載速度: {:.2} Mbps", speed);
            println!("下載完成，耗時: {:.2} 秒", duration);
        }
        Err(e) => {
            spinner.finish_and_clear();
            println!("{}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use mockito;

    use crate::download_speed;

    #[tokio::test]
    async fn test_download_speed() {
        let mut server = mockito::Server::new();

        server.mock("GET", "/test_200m.zip")
            .with_status(200)
            .with_body("fake body content")
            .create();

        let url = &format!("{}/test_200m.zip", server.url());

        let (speed, duration) = download_speed(url).await.unwrap();
        
        assert!(speed > 0.0);
        assert!(duration > 0.0);
    }
}
