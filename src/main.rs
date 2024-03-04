use bytes::Bytes;
use error_chain::error_chain;
use once_cell::sync::Lazy;
use rust_package_downloader::ThreadPool;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::sync::atomic::{
    AtomicUsize,
    Ordering::{Acquire, Release},
};

const LIST_FILE_PATH: &str = "list.txt";
const DIR_PATH: &str = "packages/";
const POOL_SIZE: usize = 8;
static NUMBER_OF_THREAD: AtomicUsize = AtomicUsize::new(0);
static CONTENTS: Lazy<String> = Lazy::new(|| {
    let contents = fs::read_to_string(LIST_FILE_PATH).expect("Couldn't read the file");
    contents
});

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = ThreadPool::new(POOL_SIZE);
    fs::create_dir_all(DIR_PATH)?;

    let rows: Vec<&str> = CONTENTS.split("\n").collect::<Vec<&str>>();
    let len = rows.len();

    for row in rows[..len - 1].iter() {
        let package_info = row.split_whitespace().collect::<Vec<&str>>();
        pool.execute(move || {
            count_up();
            let url = package_info[0];
            let filename = package_info[1];
            let mut dest = File::create(format!("{}/{}", DIR_PATH, filename))
                .unwrap_or_else(|why| panic!("Couldn't creat file {}: {}", filename, why));
            async {
                let response = reqwest::get(&url[1..url.len() - 1]).await;
                let content: Bytes = response
                    .expect("There no Response!")
                    .bytes()
                    .await
                    .expect("Couldn't cast content to string");
                copy(&mut content.as_ref(), &mut dest).expect("Couldn't write content into {}");
            };
            count_down();
        });
    }

    while NUMBER_OF_THREAD.load(Acquire) != 0 {}

    println!("All files downloaded successfully!");

    Ok(())
}
fn count_up() {
    NUMBER_OF_THREAD.fetch_add(1, Release);
}

fn count_down() {
    NUMBER_OF_THREAD.fetch_sub(1, Release);
}
