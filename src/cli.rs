use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub fumenpath: std::path::PathBuf,
    pub songpath: std::path::PathBuf,
    #[arg(short = 'd', long = "delay")]
    pub delay: Option<i32>,
    #[arg(short = 's', long = "start")]
    pub starttime: Option<u32>,
    #[arg(short = 'm', long = "meta")]
    pub metapath: Option<std::path::PathBuf>,
}