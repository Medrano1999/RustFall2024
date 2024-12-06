extern crate serde;
extern crate ureq;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod worker;

fn main() {
    // Configuration
    let websites = vec![
        "https://example.com".to_string(),
        "https://rust-lang.org".to_string(),
        "https://google.com".to_string(),
    ];
    let num_workers = 4;
    let timeout = Duration::from_secs(5);

    // Create a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // Divide websites into chunks for each worker
    let task_chunks: Vec<Vec<String>> = websites
        .chunks((websites.len() + num_workers - 1) / num_workers)
        .map(|chunk| chunk.to_vec())
        .collect();

    // Spawn threads to process website chunks
    for (id, chunk) in task_chunks.into_iter().enumerate() {
        let tx_clone = tx.clone(); // Clone the transmitter for each thread
        thread::spawn(move || {
            worker::process_websites(id, chunk, tx_clone, timeout);
        });
    }

    // Close the transmitting end to signal completion
    drop(tx);

    // Collect and print results from the receiver
    println!("Website Monitoring Results:");
    while let Ok(status) = rx.recv() {
        println!("{:?}", status);
    }
}
