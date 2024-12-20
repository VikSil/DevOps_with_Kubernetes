use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use http::Method;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, Serialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

#[derive(Clone, Serialize)]
struct TodoList {
    todos: Vec<Todo>,
    image_url: String,
}

#[tokio::main]
async fn main() {
    // Shared state for the todos
    let todos: Arc<Mutex<Vec<Todo>>> = Arc::new(Mutex::new(vec![]));

    let address: String = String::from("0.0.0.0:3040");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/todos", get(get_todos).post(post_todo))
        .layer(ServiceBuilder::new().layer(cors))
        .layer(Extension(todos))
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}

async fn get_todos(Extension(todos): Extension<Arc<Mutex<Vec<Todo>>>>) -> impl IntoResponse {
    replace_image().await.expect("image refresh failed");

    let image_filepath: &str = "picture.txt";

    let return_url: String = match fs::read_to_string(&image_filepath).await {
        Ok(data) => data,
        Err(_) =>
            format!(
                "https://fastly.picsum.photos/id/633/1200/1200.jpg?hmac=w3wSzGHuyT-aMKInisjPvciLC7gIgyXaBAeU7nzo-c4"
            ),
    };

    let return_todos = todos.lock().unwrap();
    let return_todolist = TodoList {
        todos: return_todos.clone(),
        image_url: return_url,
    };

    Json(return_todolist).into_response()
}

async fn post_todo(
    Extension(todos): Extension<Arc<Mutex<Vec<Todo>>>>,
    Json(add_todo): Json<NewTodo>,
) -> Json<Todo> {
    let mut existing_todos = todos.lock().unwrap();
    let next_id = (existing_todos.len() as u32) + 1;
    let new_todo = Todo {
        id: next_id,
        title: add_todo.title,
        completed: false,
    };
    existing_todos.push(new_todo.clone());
    Json(new_todo)
}

async fn replace_image() -> Result<(), Error> {
    let image_filepath: &str = "/usr/local/files/picture.txt";
    let time_filepath: &str = "/usr/local/files/timestamp.txt";
    let previous_time: u64 = match fs::read_to_string(&time_filepath).await {
        Ok(data) => data.parse::<u64>().unwrap(),
        Err(_) => 0,
    };

    let timeout: u64 = match env::var("TIMEOUT") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(_e) => 3600, // by default serve a new image every hour
    };

    let now = SystemTime::now();
    // Get time since start of epoch in seconds
    let current_time = now
        .duration_since(UNIX_EPOCH)
        .expect("Time malfunctioned")
        .as_secs();

    if previous_time + timeout < current_time {
        let url = "https://picsum.photos/1200";

        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    let mut image_file = fs::File::create(image_filepath).await.unwrap();
                    image_file
                        .write_all(response.url().to_string().as_bytes())
                        .await
                        .expect("Error while saving image url");

                    let mut time_file = fs::File::create(time_filepath).await.unwrap();
                    time_file
                        .write_all(current_time.to_string().as_bytes())
                        .await
                        .expect("Error while saving timestamp");
                }
            }
            Err(e) => eprintln!("Request error: {}", e),
        }
    }
    Ok(())
}
