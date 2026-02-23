use clap::Parser;
use std::net::{Ipv4Addr, TcpStream};
use std::path::{Path, PathBuf};

const TARGET_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 2, 74);
const TARGET_PORT: &str = "6767";
// TODO: Maybe use ipv6?

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Weather report written in Rust using OpenWeather",
    override_usage = "west <COUNTRY PROVINCE CITY FORMAT>"
)]

struct Args {
    ///
    #[arg(long)]
    folder: PathBuf,
}

fn main() {
    let args = Args::parse();
    let folder_str = args.folder.to_str().unwrap();

    let target_net: String = format!("{TARGET_IP}:{TARGET_PORT}");
    let target_dir: PathBuf = Path::new(folder_str).to_path_buf();

    let mut stream = TcpStream::connect("{TARGET_IP}");

    println!("Using: {:?}", target_dir);
}
