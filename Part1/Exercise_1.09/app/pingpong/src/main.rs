use axum::{Extension, Router, routing::get };
use std::sync::{Arc, Mutex};


#[tokio::main]
async fn main() {
    let visit_count = Arc::new(Mutex::new(-1));

    let address: String = String::from("0.0.0.0:3033");

    let router = Router::new().route("/pingpong", get(index)).layer(Extension(visit_count)).into_make_service();

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());
    // axum::serve(listener, router).await.unwrap();
    axum::serve(listener,router).await.unwrap();
}

async fn index(Extension(visit_count): Extension<Arc<Mutex<i32>>>) ->  String {
    let mut count = visit_count.lock().unwrap();
    *count += 1;
    format!("pong {}", count)
}

