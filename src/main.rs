use error_chain::error_chain;
use std::fs;
use std::fs::File;
use std::io::copy;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = "list.txt";
    let dir_path = "packages/";

    fs::create_dir_all(dir_path)?;

    let urls = fs::read_to_string(file_path).expect("cannnot read the file");
    let mut https: Vec<&str> = urls.split("\n").collect();

    https.pop();
    for package in https {
        let package_info = package.split_whitespace().collect::<Vec<&str>>();
        let url = package_info[0];
        let fname = package_info[1];
        let mut dest = File::create(format!("{}/{}", dir_path, fname))?;
        let response = reqwest::get(&url[1..url.len() - 1]).await?;
        let content = response.bytes().await?;
        copy(&mut content.as_ref(), &mut dest)?;
    }

    println!("All files downloaded successfully!");

    Ok(())
}
