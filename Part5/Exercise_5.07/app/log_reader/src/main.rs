use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::env;
use tokio::fs;

#[tokio::main]
async fn main() {
    // Set up Web server
    let mut address: String = String::from("0.0.0.0:3022");
    match env::var("READ_PORT") {
        Ok(val) => address = address.replace("3022", &val),
        Err(_e) => {
            println!("Environment variable READ_PORT not defined. Using default port 3022")
        }
    };
    let router = Router::new()
        .route("/", get(all_ok))
        .route("/logreader", get(index))
        .route("/healthz", get(healthcheck));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

// Serve to / route
async fn index() -> String {
    let pingpong_host: String = match env::var("PINGPONG_HOST") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable PINGPONG_HOST is not defined."),
    };
    let log_host: String = match env::var("LOG_HOST") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable LOG_HOST is not defined."),
    };

    let pingpong_url: String = String::from("http://") + &pingpong_host;
    let pongs: String = get_response(&(pingpong_url + "/pingpong")).await;

    let log_url: String = String::from("http://") + &log_host;
    let log_output: String = get_response(&log_url).await;

    let config_file: &str = "/usr/local/config/information.txt";
    let info: String = fs::read_to_string(&config_file).await.unwrap();

    let message: String = match env::var("MESSAGE") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable MESSAGE is not defined. Check ConfigMap"),
    };

    let output: String = "file content: ".to_string()
        + &info
        + "env variable: MESSAGE="
        + &message
        + "\n"
        + &log_output
        + "\n"
        + &pongs;

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

async fn all_ok() -> impl IntoResponse {
    println!("Received a diagnostic request");

    (StatusCode::OK, "All is OK").into_response()
}

async fn healthcheck() -> impl IntoResponse {
    println!("Received a healthz request");

    let backend_host: String = match env::var("PINGPONG_HOST") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable PINGPONG_HOST is not defined."),
    };
    let url: String = String::from("http://") + &backend_host + "/pingpong";

    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                println!("Connection established");
                (StatusCode::OK, "Connection established").into_response()
            } else {
                println!("Unaccessible");
                (StatusCode::from_u16(500).unwrap(), "Unaccessible").into_response()
            }
        }
        Err(_e) => {
            println!("Unaccessible");
            (StatusCode::from_u16(500).unwrap(), "Unaccessible").into_response()
        }
    }
}
