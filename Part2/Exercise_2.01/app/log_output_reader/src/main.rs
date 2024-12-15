use axum::{ Router, routing::get };

#[tokio::main]
async fn main() {
    // Set up Web server
    let address: String = String::from("0.0.0.0:3022");
    let router = Router::new().route("/", get(index));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

// Serve to / route
async fn index() -> String {
    let log_output: String = get_response("http://log-output-service:3011").await;
    let pongs: String = get_response("http://pingpong-service:3033/pingpong").await;

    let output: String = log_output + "\n" + &pongs;

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
