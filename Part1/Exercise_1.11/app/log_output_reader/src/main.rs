use axum::{ Router, routing::get };
use rev_buf_reader::RevBufReader;
use std::io::BufRead;
use std::fs::File;
use tokio::fs;

#[tokio::main]
async fn main() {
    // Set up Web server
    let address: String = String::from("0.0.0.0:3011");
    let router = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

// Serve to / route
async fn index() -> String {
    // Get last line from the log output file
    let path: &str = "/usr/local/files/output.txt";
    let file = File::open(&path).expect("File not found");
    let buffer = RevBufReader::new(&file);
    let log_output: String = buffer
        .lines()
        .take(1)
        .map(|l| l.expect("Failed to parse the file"))
        .collect();

    // Get number of pongs
    let path2: &str = "/usr/local/files/pongs.txt";
    let pongs: String = fs::read_to_string(&path2).await.unwrap();

    let output: String = log_output + "\nPing / Pongs: " + &pongs;

    format!("{}", output) // respond to GET request
}
