use std::net::Ipv4Addr;

use clap::Parser;

const TARGET_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 2, 74);
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
    fahrenheit: bool,
}

fn main() {
    println!("{:?}", TARGET_IP);
}
