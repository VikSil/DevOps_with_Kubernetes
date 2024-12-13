use chrono::prelude::{ DateTime, Utc };
use random_string::generate;
use std::{ fs, io::Write, thread, time };

fn append_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::OpenOptions
        ::new()
        .create(true) //create file if it does not exist
        .append(true) // writ ein append mode
        .open(&path)?;

    file.write_all(&data.as_bytes())?;
    file.write_all("\n".as_bytes())?;

    Ok(())
}

fn main() {
    let charset: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let timeout = time::Duration::from_millis(5_000);

    loop {
        let s: String = String::from(generate(37, charset));
        let now = time::SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let concatenated: String = now.to_string() + ": " + &s;

        match append_file(&"/usr/local/files/output.txt", &concatenated) {
            Ok(_) => println!("{}", concatenated),
            Err(_) => println!("An error occured while outputing to the file"),
        }
        thread::sleep(timeout);
    }
}
