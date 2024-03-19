use bytes::Bytes;
use error_chain::error_chain;
use once_cell::sync::Lazy;
use rust_package_downloader::ThreadPool;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;

const LIST_FILE_PATH: &str = "list.txt";
const DIR_PATH: &str = "packages/";
const POOL_SIZE: usize = 4;

static CONTENTS: Lazy<String> =
    Lazy::new(|| fs::read_to_string(LIST_FILE_PATH).expect("Couldn't read the file"));

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut pool = ThreadPool::new(POOL_SIZE);
    fs::create_dir_all(DIR_PATH)?;

    let rows: Vec<&str> = CONTENTS.split('\n').collect();
    let len = rows.len();

    for row in &rows[..len - 1] {
        let package_info: Vec<&str> = row.split_whitespace().collect();
        pool.execute(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async move {
                for i in 0..5 {
                    match download_file(&package_info).await {
                        Ok(_) => break,
                        Err(e) => println!(
                            "{} Coudn't download {}: {}",
                            "-".repeat(i + 1),
                            package_info[1],
                            e
                        ),
                    }
                }
            });
        });
    }

    pool.join();
    println!("All files downloaded successfully!");
    Ok(())
}

async fn download_file(package_info: &Vec<&str>) -> Result<()> {
    let url = package_info[0];
    let filename = package_info[1];
    let dest_path = Path::new(DIR_PATH).join(filename);
    let mut dest = File::create(dest_path)?;

    let response = reqwest::get(&url[1..url.len() - 1]).await?;
    let content: Bytes = response.bytes().await?;

    copy(&mut content.as_ref(), &mut dest)?;
    Ok(())
}
