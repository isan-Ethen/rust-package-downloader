mod downloader;

fn main() {
    let mut downloader = downloader::Downloader::new("list.txt", "packages/", None);
    downloader.run();
    downloader.print_result();
}
