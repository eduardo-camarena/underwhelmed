mod args;

use clap::Parser;
use indicatif::{ProgressIterator, ProgressStyle};
use std::{fs, io};

use args::UnderwhelmingArgs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = UnderwhelmingArgs::parse();

    for i in (args.first - 1..args.last).progress().with_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}")
            .unwrap()
            .progress_chars("##-"),
    ) {
        let mut url = format!("{}/{}", args.base_url, i + 1);
        add_extra_url_options(&mut url, &args);
        let response = reqwest::get(url).await;
        if response.is_ok() {
            let file = response.unwrap().bytes().await;
            let file_name = get_file_name(i + 1, &args);
            if file.is_ok() && file_name.is_ok() {
                fs::write(file_name.unwrap(), file.unwrap())
                    .expect("there was an error while saving the file");
            }
        }
    }

    Ok(())
}

fn add_extra_url_options(url: &mut String, args: &UnderwhelmingArgs) {
    if args.add_extention {
        url.push_str(format!(".{}", args.ext).as_str());
    }
    if args.query.is_some() {
        url.push_str(format!("?{}", args.query.as_ref().unwrap()).as_str());
    }
}

fn get_file_name(image_number: u32, args: &UnderwhelmingArgs) -> io::Result<String> {
    let mut file_name = format!("{}/{}", env!("UNDERWHELMED_BASE_PATH"), args.destionation);

    file_name.push_str(
        match args.name.as_ref() {
            Some(name) => format!("{} ({}).{}", image_number, name, args.ext),
            None => format!("{}.{}", image_number, args.ext),
        }
        .as_str(),
    );

    Ok(file_name)
}
