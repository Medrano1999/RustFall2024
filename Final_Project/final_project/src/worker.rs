use std::sync::mpsc::Sender;
use chrono::{DateTime, Utc};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

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
        let status = match ureq::get(&url).timeout(timeout).call() {
            Ok(response) => Ok(response.status()),
            Err(err) => Err(err.to_string()),
        };
        let response_time = start.elapsed();

        let website_status = WebsiteStatus {
            url: url.clone(),
            status,
            response_time,
            timestamp: Utc::now(),
        };

        // Send the status to the main thread
        if let Err(err) = tx.send(website_status) {
            eprintln!("Worker {}: Failed to send status for {}: {}", worker_id, url, err);
        }
    }
}
