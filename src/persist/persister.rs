use std::io::{Error, BufRead};
use crate::utils::file;
use monoio::io::{BufWriter, BufReader};

pub trait Persister {
    async fn save(&self, path: &str, bytes: Vec<u8>) -> Result<(), Error>;
    async fn load(&self, path: &str) -> Result<String, Error>;
}

pub struct FilePersister {}

impl Persister for FilePersister {
    async fn save(&self, path: &str, bytes: Vec<u8>) -> Result<(), Error> {
        let file = file::create_file(path).await?;
        let writer = BufWriter::new(file);
        let xd = writer.into_inner();
        let res = xd.write_at(bytes,0).await;
        Ok(())
    }

    async fn load(&self, path: &str) -> Result<String, Error> {
        todo!()
    }
}
