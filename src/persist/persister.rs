use std::io::Error;
use crate::utils::file;


pub struct FilePersister {}
impl FilePersister {
    async fn save(&self, path: &str, bytes: Vec<u8>) -> Result<(), Error> {
        let mut file = file::create_file(path).await?;
        file.write_all_at(bytes, 0).await.0.map_err(|e| e.into())
    }

    async fn load(&self, path: &str) -> Result<String, Error> {
        todo!()
    }
}
