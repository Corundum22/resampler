pub enum FileData {
    CSV,
    Terminal,
    Nothing,
}

impl FileData {
    pub fn from_string(val: &str) -> FileData {
        match val.split('.').last().unwrap() {
            "csv" => FileData::CSV,
            "terminal" | "console" | "tty" => FileData::Terminal,
            _ => FileData::Nothing,
        }
    }
}
