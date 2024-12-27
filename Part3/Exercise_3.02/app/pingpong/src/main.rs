use axum::{http::StatusCode, response::IntoResponse, routing::get,  Router};
use sqlx::Row;
use std::env;
use std::error::Error;

struct Pings {
    pub id: i32,
    pub pongs: i32,
}

async fn read(conn: &sqlx::PgPool) -> Result<Pings, Box<dyn Error>> {
    let q = "SELECT * FROM pings WHERE id = 1";
    let query = sqlx::query(q);

    let row = query.fetch_one(conn).await.unwrap();
    let pings = Pings {
        id: row.get("id"),
        pongs: row.get("pongs"),
    };

    Ok(pings)
}

async fn write(pongs: &i32, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE pings SET pongs = $1 WHERE id = 1";

    sqlx::query(query).bind(&pongs).execute(conn).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    let url: String = match env::var("DB_URL") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable DB_URL is not defined."),
    };

    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let address: String = String::from("0.0.0.0:3033");

    let router = Router::new()
        .route("/", get(all_ok))
        .route("/pingpong", get(index))
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Server started on port {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}

async fn index() -> String {
    let url: String = match env::var("DB_URL") {
        Ok(val) => val,
        Err(_e) => String::from("Environment variable DB_URL is not defined."),
    };
    let pool = sqlx::postgres::PgPool::connect(&url).await.unwrap();

    let mut pongs = match read(&pool).await {
        Ok(data) => data.pongs,
        Err(_) => -1,
    };
    println!("Retrieved {} pongs from the DB", pongs);

    pongs += 1;

    // write to the DB
    if let Err(e) = write(&pongs, &pool).await {
        eprintln!("Failed to write to DB: {}", e);
        return format!("Error: {}", e);
    }

    // output to user
    format!("Ping / Pongs: {}", pongs)
}

async fn all_ok()  -> impl IntoResponse  {
    println!("Received a diagnostic request");

    (StatusCode::OK,"All is OK").into_response()
}
