use monoio::fs::{File, OpenOptions};

pub async fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().create(true).write(true).open(path).await
}