use axum::{ Router, routing::get };
use std::env;

#[tokio::main]
async fn main() {
    let mut address: String = String::from("0.0.0.0:3030");

    match env::var("PORT") {
        Ok(val) => {
            address = address.replace("3030", &val);
        }
        Err(_e) => println!("Environment variable PORT not defined. Using default port 3030"),
    }

    let router = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

async fn index() -> String {
    format!("Nothing to see here yet")
}
