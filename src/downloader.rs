mod downloadresult;
mod threadpool;

use downloadresult::DownloadResult;
use threadpool::ThreadPool;

use error_chain::error_chain;
use num_cpus;
use reqwest::{self, Client};
use std::{
    fs,
    fs::File,
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub struct Downloader {
    package_list: Vec<String>,
    directory_path: String,
    threadpool: ThreadPool,
    status: Arc<Mutex<DownloadResult>>,
    package_num: Arc<usize>,
}

impl Downloader {
    pub fn new(filename: &str, directory_path: &str, poolsize: Option<usize>) -> Downloader {
        let package_list: Vec<String> = fs::read_to_string(filename)
            .expect("Couldn't read the file")
            .lines()
            .map(String::from)
            .collect();
        let package_num: usize = package_list.len();

        let poolsize = if let Some(size) = poolsize {
            size
        } else {
            num_cpus::get()
        };

        Downloader {
            package_list,
            directory_path: directory_path.to_string(),
            threadpool: ThreadPool::new(poolsize),
            status: Arc::new(Mutex::new(DownloadResult::new())),
            package_num: Arc::new(package_num),
        }
    }

    pub fn run(&mut self) {
        fs::create_dir_all(&self.directory_path).expect("Couldn't create directory");
        let client = Arc::new(Client::new());

        for package in &self.package_list {
            let package_info: Vec<String> = package.split_whitespace().map(String::from).collect();
            let directory_path = self.directory_path.clone();
            let failed_packages = Arc::clone(&self.status);
            let package_num = Arc::clone(&self.package_num);
            let client = client.clone();

            self.threadpool.execute(move || {
                Downloader::download_files(
                    package_info,
                    directory_path,
                    failed_packages,
                    package_num,
                    client,
                );
            });
        }

        self.threadpool.join();
    }

    fn download_files(
        package_info: Vec<String>,
        directory_path: String,
        failed_packages: Arc<Mutex<DownloadResult>>,
        package_num: Arc<usize>,
        client: Arc<Client>,
    ) {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move {
            let mut cnt = 0;
            let url = &package_info[0];
            let file_name = &package_info[1];

            loop {
                if Downloader::handle_result(
                    Downloader::download_file(&directory_path, &url, &file_name, &client).await,
                    &cnt,
                    &file_name,
                    &url,
                ) {
                    break;
                }
                cnt += 1;
                if cnt >= 5 {
                    Downloader::update_status(failed_packages, &file_name, package_num);
                    break;
                }
            }
        });
    }

    async fn download_file(
        directory_path: &str,
        url: &String,
        file_name: &String,
        client: &Arc<Client>,
    ) -> Result<()> {
        let mut response = client.get(&url[1..url.len() - 1]).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err("File not found".into());
        }

        let dest_path = Path::new(directory_path).join(file_name);
        let mut dest = File::create(dest_path)?;

        while let Some(chunk) = response.chunk().await? {
            dest.write_all(&chunk)?;
        }
        Ok(())
    }

    fn handle_result(result: Result<()>, cnt: &usize, file_name: &String, url: &String) -> bool {
        match result {
            Ok(_) => {
                println!("Download {} success!", file_name);
                true
            }
            Err(e) => {
                eprintln!(
                    "{} Couldn't download {} from {}: {}",
                    cnt + 1,
                    file_name,
                    url,
                    e
                );
                false
            }
        }
    }

    fn update_status(
        failed_packages: Arc<Mutex<DownloadResult>>,
        file_name: &String,
        package_num: Arc<usize>,
    ) {
        loop {
            if let Ok(mut guard) = failed_packages.lock() {
                guard.add(file_name.clone());
                if *package_num == guard.len() {
                    guard.change_to_failed();
                }
                break;
            }
        }
    }

    pub fn print_result(&self) {
        println!();
        (*self.status).lock().unwrap().print_result();
    }
}
