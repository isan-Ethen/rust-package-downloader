mod downloader;

fn main() {
    let mut downloader = downloader::Downloader::new("list.txt", "packages/", 4);
    downloader.run();
    downloader.print_result();
}
