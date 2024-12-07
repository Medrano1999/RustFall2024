mod worker; // Include worker module
mod utils;  // Include utils module

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let websites = vec![
        "https://example.com".to_string(),
        "https://rust-lang.org".to_string(),
        "https://google.com".to_string(),
    ];

    let num_workers = 3;
    let (tx, rx) = mpsc::channel();

    // Divide the websites into chunks
    let chunks = utils::chunkify(&websites, num_workers);

    // Spawn threads for workers
    for (id, chunk) in chunks.into_iter().enumerate() {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            worker::process_websites(id, chunk, tx_clone, Duration::from_secs(5));
        });
    }

    // Collect results from workers
    println!("Website Monitoring Results:");
    while let Ok(status) = rx.recv() {
        println!("{:?}", status);
    }
}
