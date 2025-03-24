use codemap::codemap;
use ignore::{WalkBuilder, types::TypesBuilder};
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() {
    let types = TypesBuilder::new()
        .add_defaults()
        .select("rust")
        .build()
        .unwrap();

    let walker = WalkBuilder::new("./").types(types).build();

    for result in walker {
        match result {
            Ok(entry) => {
                if entry.path().is_dir() {
                    continue;
                }

                let mut file = File::open(entry.path()).await.unwrap();
                let mut contents = String::new();

                // Read the file contents into the string
                file.read_to_string(&mut contents).await.unwrap();

                let codemap = codemap(&contents);

                println!("{}", codemap);
            }
            Err(err) => println!("ERROR: {}", err),
        }
    }
}
