use async_recursion::async_recursion;
use std::fs;
use std::path::Path;
use tracing::{debug, info};

pub async fn convert(directory_string: String) {
    info!("{}", &directory_string);

    rename_or_open_dir(directory_string).await;

    println!("JS to TS complete!")
}

#[async_recursion]
async fn rename_or_open_dir(directory_string: String) {
    let directory = Path::new(&directory_string);
    match directory.is_dir() {
        true => {
            debug!("{:?} is a directory", &directory);
            let files = fs::read_dir(directory)
                .unwrap()
                .map(|res| res.map(|e| fs::canonicalize(e.path()).unwrap()))
                .collect::<Result<Vec<_>, std::io::Error>>()
                .unwrap();

            info!("{:?}", &files);

            for file in files {
                let file_string = file.to_string_lossy().to_string();
                tokio::spawn(async move {
                    rename_or_open_dir(file_string).await
                });
            }
        }
        false => {
            // check for .js file
            match directory.extension() {
                Some(file_ext) => {
                    match file_ext == "js" {
                        true => {
                            debug!("{} is .js file, converting...", &directory.to_string_lossy());
                            let file = directory.with_extension("ts");
                            fs::rename(directory, file).unwrap();
                        }
                        false => {
                            debug!(
                                "{} is not .js file, skipping...",
                                directory.to_string_lossy()
                            );
                        }
                    };
                },
                None => ()
            };
        }
    };
}
