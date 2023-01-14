use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// The path to the fumen file, supports ply and json
    pub fumenpath: std::path::PathBuf,
    /// The path to the song file, supports mp3 and ogg
    pub songpath: std::path::PathBuf,
    /// The delay in milliseconds between the start of the song and the start of the fumen
    #[arg(short = 'd', long = "delay")]
    pub delay: Option<i32>,
    /// Make the song start at this time in miliseconds instead of the beginning
    #[arg(short = 's', long = "start")]
    pub starttime: Option<u32>,
    /// The path to the metadata file, not supported yet!
    #[arg(short = 'm', long = "meta")]
    pub metapath: Option<std::path::PathBuf>,
}