mod args;

use clap::Parser;
use indicatif::{ProgressIterator, ProgressStyle};
use std::fs;

use args::UnderwhelmingArgs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = UnderwhelmingArgs::parse();

    for i in (0..args.pages).progress().with_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}")
            .unwrap()
            .progress_chars("##-"),
    ) {
        let url = format!("{}/{}.jpg", args.base_url, i + 1);
        let response = reqwest::get(url).await;
        if response.is_ok() {
            let file = response.unwrap().bytes().await;
            if file.is_ok() {
                fs::write(
                    format!("{}/{}.{}", args.dest, i, args.extension),
                    file.unwrap(),
                )
                .expect("there was an error while saving the file");
            }
        }
    }

    Ok(())
}
