use std::{thread, time};
use random_string::generate;
use chrono::prelude::{DateTime, Utc};

const TIMEOUT_MS: u64 =  5_000;
const STRING_LENGTH: usize = 37;
const CHARSET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn main() {
    let mut i: i32 = 1;
    let timeout = time::Duration::from_millis(TIMEOUT_MS);
    let s: String = String::from(generate(STRING_LENGTH, CHARSET));

    while i < 10 {
        let now = time::SystemTime::now();
        let now: DateTime<Utc> = now.into();

        println!("{}: {}",now, s);
        i += 1;
        thread::sleep(timeout);
    }
}