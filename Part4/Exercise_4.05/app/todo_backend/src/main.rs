use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use http::Method;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use std::env;
use std::error;
use std::time::{SystemTime, UNIX_EPOCH};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, FromRow, Serialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct NewTodo {
    title: String,
}

#[derive(Deserialize)]
struct CompletedTodo {
    id: i32,
    completed: bool,
}

#[derive(Clone, Serialize)]
struct TodoList {
    todos: Vec<Todo>,
    image_url: String,
}

#[tokio::main]
async fn main() {
    let url: String = match env::var("DB_URL") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable DB_URL is not defined."),
    };

    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();
    let migration = match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_data) => format!("Migration successful"),
        Err(e) => format!("Migration failed because {}", e),
    };

    println!("{}", migration);

    let mut address: String = String::from("0.0.0.0:3040");

    match env::var("PORT") {
        Ok(val) => {
            address = address.replace("3040", &val);
        }
        Err(_e) => println!("Environment variable PORT not defined. Using default port 3040"),
    }

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_origin(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/", get(all_ok))
        .route("/todos", get(get_todos).post(post_todo).put(put_todo))
        .route("/healthz", get(healthcheck))
        .layer(ServiceBuilder::new().layer(cors))
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}

async fn get_todos() -> impl IntoResponse {
    println!("Received a GET request");

    let url: String = match env::var("DB_URL") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable DB_URL is not defined."),
    };

    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    replace_image(&pool).await.expect("image refresh failed");

    let return_url: String = match read_image(&pool).await {
        Ok(data) => data,
        Err(_) =>
            format!(
                "https://fastly.picsum.photos/id/633/1200/1200.jpg?hmac=w3wSzGHuyT-aMKInisjPvciLC7gIgyXaBAeU7nzo-c4"
            ),
    };

    let return_todos = read_todos(&pool).await.unwrap();
    let return_todolist = TodoList {
        todos: return_todos.clone(),
        image_url: return_url,
    };

    Json(return_todolist).into_response()
}

async fn post_todo(Json(recieved_todo): Json<NewTodo>) -> impl IntoResponse {
    println!("Received a POST request");

    let url: String = match env::var("DB_URL") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable DB_URL is not defined."),
    };

    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    let title = recieved_todo.title;

    if title.len() <= 140 {
        println!("Valid request, new TODO: {}", title);

        let add_todo = NewTodo { title: title };
        let new_todo = writetodo(add_todo, &pool).await.unwrap();

        Json(new_todo).into_response()
    } else {
        println!("Invalid request, TODO too long: {}", title);
        (StatusCode::BAD_REQUEST, "Invalid request data").into_response()
    }
}

async fn put_todo(Json(recieved_todo): Json<CompletedTodo>) -> impl IntoResponse {
    println!("Received a PUT request");

    let url: String = match env::var("DB_URL") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable DB_URL is not defined."),
    };

    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    let id: i32 = recieved_todo.id;
    let completed: bool = recieved_todo.completed;

    let complete_todo = CompletedTodo {
        id: id,
        completed: completed,
    };
    let completed_todo = marktodo(complete_todo, &pool).await.unwrap();

    Json(completed_todo).into_response()
}

async fn replace_image(pool: &sqlx::PgPool) -> Result<(), Error> {
    let previous_time: u64 = match read_time(&pool).await {
        Ok(data) => data,
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
        println!("Time to change the picture");
        let url = "https://picsum.photos/1200";

        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    write_time_and_url(current_time, &response.url().to_string(), &pool)
                        .await
                        .unwrap();
                }
            }
            Err(e) => eprintln!("Request error: {}", e),
        }
    }
    Ok(())
}

async fn read_todos(pool: &sqlx::PgPool) -> Result<Vec<Todo>, Box<dyn error::Error>> {
    let q = "SELECT * FROM todolist";
    let query = sqlx::query(q);

    let rows = query.fetch_all(pool).await.unwrap();

    let todolist = rows
        .iter()
        .map(|row| Todo {
            id: row.get("id"),
            title: row.get("title"),
            completed: row.get("completed"),
        })
        .collect();

    Ok(todolist)
}

async fn writetodo(todo: NewTodo, pool: &sqlx::PgPool) -> Result<Todo, Box<dyn error::Error>> {
    let query = "INSERT INTO todolist (title,completed) VALUES ($1,$2) RETURNING *";

    match sqlx::query_as::<_, Todo>(query)
        .bind(&todo.title)
        .bind(false)
        .fetch_one(pool)
        .await
    {
        Ok(new_todo) => Ok(new_todo),
        Err(e) => Err(Box::new(e)),
    }
}

async fn read_time(pool: &sqlx::PgPool) -> Result<u64, Box<dyn error::Error>> {
    let q = "SELECT time FROM image WHERE id = 1";
    let query = sqlx::query(q);

    let row = query.fetch_one(pool).await.unwrap();
    let time: i64 = row.get("time");
    let converted_time = time as u64;

    Ok(converted_time)
}

async fn marktodo(todo: CompletedTodo, pool: &sqlx::PgPool) -> Result<Todo, Box<dyn error::Error>> {
    let query = "UPDATE todolist SET completed = $1 WHERE id = $2 RETURNING *";

    match sqlx::query_as::<_, Todo>(query)
        .bind(&todo.completed)
        .bind(&todo.id)
        .fetch_one(pool)
        .await
    {
        Ok(completed_todo) => Ok(completed_todo),
        Err(e) => Err(Box::new(e)),
    }
}

async fn read_image(pool: &sqlx::PgPool) -> Result<String, Box<dyn error::Error>> {
    let q = "SELECT url FROM image WHERE id = 1";
    let query = sqlx::query(q);

    let row = query.fetch_one(pool).await.unwrap();
    let url: String = row.get("url");

    Ok(url)
}

async fn write_time_and_url(
    time: u64,
    url: &str,
    pool: &sqlx::PgPool,
) -> Result<(), Box<dyn error::Error>> {
    let query = "UPDATE image SET time = $1, url = $2 WHERE id = 1";

    sqlx::query(query)
        .bind(time as i64)
        .bind(url)
        .execute(pool)
        .await
        .unwrap();

    println!("New picture URL: {}", url);

    Ok(())
}

async fn all_ok() -> impl IntoResponse {
    println!("Received a diagnostic request");

    (StatusCode::OK, "All is OK").into_response()
}

async fn healthcheck() -> impl IntoResponse {
    println!("Received a healthz request");

    let url: String = match env::var("DB_URL") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable DB_URL is not defined."),
    };

    match sqlx::postgres::PgPool::connect(&url).await {
        Ok(_data) => {
            println!("DB connection established");
            (StatusCode::OK, "DB connection established").into_response()
        }
        Err(_) => {
            println!("DB unaccessible");
            (StatusCode::from_u16(500).unwrap(), "DB unaccessible").into_response()
        }
    }
}
