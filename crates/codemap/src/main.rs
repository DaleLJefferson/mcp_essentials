use codemap::codemap;
use ignore::{WalkBuilder, types::TypesBuilder};
use std::env;
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or("./");

    let types = TypesBuilder::new()
        .add_defaults()
        .select("rust")
        .build()
        .unwrap();

    let walker = WalkBuilder::new(path).types(types).build();

    println!("<codemap>");

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
                let codemap = codemap.trim();

                if codemap.is_empty() {
                    continue;
                }

                let display_path = entry.path().strip_prefix(path).unwrap_or(entry.path());
                println!(
                    "<file path=\"{}\">\n{}\n</file>",
                    display_path.display(),
                    codemap
                );
            }
            Err(err) => println!("ERROR: {}", err),
        }
    }

    println!("</codemap>");
}
