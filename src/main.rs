use std::path::PathBuf;

use crate::{dns::DNSUpdater, ip::IPFinder};
use clap::Parser;

mod dns;
mod ip;

pub type Address = std::net::Ipv4Addr;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, name = "token")]
    token_file: PathBuf,

    #[clap(short, long)]
    name: String,
}

fn main() -> eyre::Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    let ip = Address::UNSPECIFIED;
    let token = std::fs::read_to_string(args.token_file)?.trim().to_string();

    let digital_ocean = dns::DigitalOcean::new(token);

    digital_ocean.set_dns(&args.name, ip)?;
    println!("set dns");

    //let ip = ip::IfConfig.find_ip()?;

    println!("Public IP Found: {ip}");

    Ok(())
}
