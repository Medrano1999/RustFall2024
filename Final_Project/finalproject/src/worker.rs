use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

pub fn process_websites(
    worker_id: usize,
    websites: Vec<String>,
    tx: Sender<WebsiteStatus>,
    timeout: Duration,
) {
    for url in websites {
        let start = Instant::now();
        let result = match ureq::get(&url).timeout(timeout).call() {
            Ok(response) => Ok(response.status()),
            Err(err) => Err(err.to_string()),
        };
        let duration = start.elapsed();
        let status = WebsiteStatus {
            url,
            status: result,
            response_time: duration,
            timestamp: Utc::now(),
        };

        if let Err(err) = tx.send(status) {
            eprintln!("Worker {} failed to send data: {}", worker_id, err);
        }
    }
}
