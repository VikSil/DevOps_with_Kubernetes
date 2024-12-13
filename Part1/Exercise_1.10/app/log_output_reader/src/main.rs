use axum::{ Router, routing::get };
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
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

    // Get file contents
    let path: &str = "/usr/local/files/output.txt";
    let content:String =fs::read_to_string(&path).await.unwrap();

    // Hash the contents
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    let hash = hasher.finish();

    let output:String = "Content Hash: ".to_owned() + &hash.to_string() + "\n\n" + "File Content:\n" + &content;
    format!("{}", output) // respond to GET request


}

