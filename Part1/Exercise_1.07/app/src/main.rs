use axum::{ Router, routing::get };
use chrono::prelude::{DateTime, Utc};
use random_string::generate;
use std::{sync::{Arc, Mutex},thread, time};

const TIMEOUT_MS: u64 =  5_000;
const STRING_LENGTH: usize = 37;
const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[tokio::main]
async fn main() {

    
    let timeout = time::Duration::from_millis(TIMEOUT_MS);

    // Shared state for the current random string
    let shared_string = Arc::new(Mutex::new(String::new()));

    // Clone the shared state for both threads
    let string_for_generator = Arc::clone(&shared_string);
    let string_for_server = Arc::clone(&shared_string);

    // Start the random string generator in a separate thread
    thread::spawn(move || {
        loop {
            let s: String = String::from(generate(STRING_LENGTH, CHARSET));
            let now = time::SystemTime::now();
            let now: DateTime<Utc> = now.into();
            let concatenated: String =  now.to_string() +": "+&s;

            // Update the shared string
            {
               // get the memory where shared_string lives
               let mut shared = string_for_generator.lock().unwrap();
               // put the newly generated concatenated string there 
                *shared = concatenated.clone();
            }

            println!("{}",concatenated); // output to console
            thread::sleep(timeout);
        }
    });


    // Set up Web server
    let address: String = String::from("0.0.0.0:3011");
    let router = Router::new().route("/", get(move || index(Arc::clone(&string_for_server))));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();

}


// Serve to / route
async fn index(shared_string: Arc<Mutex<String>>) -> String {
    // get the shared memory and clone into a local variable
    let current_string = shared_string.lock().unwrap();
    let output = current_string.clone();
    format!("{}", output) // respond to GET request
    }

