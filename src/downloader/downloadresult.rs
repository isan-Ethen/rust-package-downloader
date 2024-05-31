use std::mem;

pub enum DownloadResult {
    Success,
    Failed(Vec<String>),
    PartiallySuccess(Vec<String>),
}

impl DownloadResult {
    pub fn new() -> DownloadResult {
        DownloadResult::Success
    }

    pub fn len(&self) -> usize {
        if let DownloadResult::PartiallySuccess(missing_files) = self {
            missing_files.len()
        } else {
            0usize
        }
    }

    pub fn add(&mut self, filename: String) {
        if DownloadResult::is_success(self) {
            *self = DownloadResult::change_to_partiallysuccess();
        }
        if let DownloadResult::PartiallySuccess(missing_files) = self {
            missing_files.push(filename);
        }
    }

    fn is_success(result: &mut DownloadResult) -> bool {
        match result {
            DownloadResult::Success => true,
            _ => false,
        }
    }

    fn change_to_partiallysuccess() -> DownloadResult {
        DownloadResult::PartiallySuccess(Vec::new())
    }

    pub fn change_to_failed(&mut self) -> DownloadResult {
        match self {
            DownloadResult::PartiallySuccess(missing_files) => {
                let missing_files = mem::take(missing_files);
                mem::replace(self, DownloadResult::Failed(missing_files))
            }
            _ => DownloadResult::Failed(Vec::new()),
        }
    }

    pub fn print_result(&self) {
        match self {
            DownloadResult::Success => println!("All files downloaded successfully!"),
            DownloadResult::PartiallySuccess(missing_files) => {
                println!("Download was partially success");
                DownloadResult::print_missing_files(missing_files);
            }
            DownloadResult::Failed(missing_files) => {
                println!("Download files are failed");
                DownloadResult::print_missing_files(missing_files);
            }
        }
    }

    fn print_missing_files(missing_files: &Vec<String>) {
        println!("These are missing files");
        for file in missing_files {
            println!("{}", file);
        }
    }
}
