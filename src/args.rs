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

    /// first image to download
    #[arg(short, long, default_value_t = 1)]
    pub first: u32,

    /// amount of images you want to download
    #[arg(short, long, default_value_t = 1)]
    pub last: u32,

    /// file extension
    #[arg(long, default_value_t = String::from("jpg"))]
    pub ext: String,

    /// add extension to url you're downloading from
    #[arg(long, default_value_t = false)]
    pub add_extenstion: bool,

    /// query params to add to the url
    #[arg(long)]
    pub query: Option<String>,

    /// name to saved files
    #[arg(long)]
    pub name: Option<String>,
}
