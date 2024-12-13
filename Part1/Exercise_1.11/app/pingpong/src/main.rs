use axum::{ Extension, Router, routing::get };
use std::fs::File;
use std::io::Write;
use std::sync::{ Arc, Mutex };
use tokio::fs;

#[tokio::main]
async fn main() {
    let path: &str = "/usr/local/files/pongs.txt";
    let content: i32 = match fs::read_to_string(&path).await {
        Ok(data) => data.parse::<i32>().unwrap(),
        Err(_) => -1,
    };

    let visit_count = Arc::new(Mutex::new(content));

    let address: String = String::from("0.0.0.0:3033");

    let router = Router::new()
        .route("/pingpong", get(index))
        .layer(Extension(visit_count))
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());
    // axum::serve(listener, router).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn index(Extension(visit_count): Extension<Arc<Mutex<i32>>>) -> String {
    let mut count = visit_count.lock().unwrap();
    *count += 1;

    //write the current count to file
    let mut file = File::create(&"/usr/local/files/pongs.txt").unwrap();
    file.write_all(count.to_string().as_bytes()).expect("Failed to output pongs to file");

    // output to user
    format!("pong {}", count)
}
