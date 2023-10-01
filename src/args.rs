use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct UnderwhelmingArgs {
    /// base url from website you're trying to donwload images from
    #[arg(short, long)]
    pub base_url: String,

    /// path to folder you want to save files to
    #[arg(short, long)]
    pub dest: String,

    /// amount of pages you want to download
    #[arg(short, long, default_value_t = 1)]
    pub pages: u32,

    #[arg(short, long, default_value_t = String::from("jpg"))]
    pub extension: String,
}
