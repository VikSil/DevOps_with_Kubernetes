use axum::{http::StatusCode, response::IntoResponse, Router, routing::get };
use std::env;
use tokio::fs;

#[tokio::main]
async fn main() {
    // Set up Web server
    let address: String = String::from("0.0.0.0:3022");
    let router = Router::new().route("/", get(all_ok)).route("/logreader", get(index));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

// Serve to / route
async fn index() -> String {
    let backend_host: String = match env::var("BACKEND_HOST") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable DB_URL is not defined."),
    };
    let url:String = String::from("http://")+ &backend_host;
    let log_output: String = get_response(&url).await;
    let pongs: String = get_response(&(url+"/pingpong")).await;

    let config_file: &str = "/usr/local/config/information.txt";
    let info: String = fs::read_to_string(&config_file).await.unwrap();

    let message:String= match env::var("MESSAGE") {
        Ok(val) => {val}
        Err(_e) => {String::from("Environment variable MESSAGE is not defined. Check ConfigMap")}
    };

    let output: String = "file content: ".to_string() + &info +
                         "env variable: MESSAGE=" + &message  + "\n" +
                         &log_output + "\n" + &pongs;

    format!("{}", output) // respond to GET request
}

async fn get_response(url: &str) -> String {
    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                let body: String = response.text().await.unwrap();
                format!("{}", body)
            } else {
                format!("Failed to fetch the URL. Status: {}", response.status())
            }
        }
        Err(e) => format!("Request error: {}", e),
    }
}

async fn all_ok()  -> impl IntoResponse  {
    println!("Received a diagnostic request");

    (StatusCode::OK,"All is OK").into_response()
}