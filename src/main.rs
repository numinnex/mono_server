use persist::persister::{FilePersister, Persister};


mod persist;
mod utils;

#[monoio::main(driver = "fusion")]
async fn main() {
    let persister = FilePersister {};
    let bytes = b"asdf".to_vec();
    let result = persister.save("foo.txt", bytes).await;
    if let Err(err) = result {
        println!("Error: {:?}", err);
    }
}
