use axum::{ response::{ Html, IntoResponse }, Router, routing::get };
use reqwest::Error;
use std::env;
use std::time::{ SystemTime, UNIX_EPOCH };
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let mut address: String = String::from("0.0.0.0:3030");

    match env::var("PORT") {
        Ok(val) => {
            address = address.replace("3030", &val);
        }
        Err(_e) => println!("Environment variable PORT not defined. Using default port 3030"),
    }

    let router = Router::new()
        .nest_service("/usr/local/files", ServeDir::new("/usr/local/files"))
        .route("/", get(index));

    println!("Server started on port {}", &address);

    axum::Server::bind(&address.parse().unwrap()).serve(router.into_make_service()).await.unwrap()
}

async fn index() -> impl IntoResponse {
    replace_image().await.expect("Image download failed");

    Html(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Todo App</title>
        <style>.center {
            display: block;
            margin-left: auto;
            margin-right: auto;
            }
        </style>
    </head>
    <body>

        <img src="/usr/local/files/picture.jpg" alt="Random picture" width="300" class="center">
    </body>
    </html>
"#
    )
}

async fn download_image() -> Result<(), Error> {
    let url = "https://picsum.photos/1200";
    let file_path = "/usr/local/files/picture.jpg";

    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(content) => {
                        let mut file = fs::File
                            ::create(file_path).await
                            .expect("Failed to create file");
                        file.write_all(&content).await.expect("Failed to update image");
                    }
                    Err(e) => eprintln!("Error getting picture: {}", e),
                }
            } else {
                eprintln!("Request failed with status: {}", response.status());
            }
        }
        Err(e) => eprintln!("Request error: {}", e),
    }

    println!("Image successfully downloaded to {}", file_path);
    Ok(())
}

async fn replace_image() -> Result<(), Error> {
    let filepath: &str = "/usr/local/files/timestamp.txt";
    let previous_time: u64 = match fs::read_to_string(&filepath).await {
        Ok(data) => data.parse::<u64>().unwrap(),
        Err(_) => 0,
    };

    let timeout: u64 = match env::var("TIMEOUT") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(_e) => 3600, // by default serve a new image every hour
    };

    let now = SystemTime::now();
    // Get time since start of eposh in seconds
    let current_time = now.duration_since(UNIX_EPOCH).expect("Time malfunctioned").as_secs();

    if previous_time + timeout < current_time {
        download_image().await.expect("Image download failed");
        //write the current count to file
        let mut file = fs::File::create(&"/usr/local/files/timestamp.txt").await.unwrap();
        file.write_all(current_time.to_string().as_bytes()).await.expect(
            "Error while saving current timestamp"
        );
    } else {
        println!("Serving the same image");
    }

    Ok(())
}
