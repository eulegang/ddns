use std::path::PathBuf;

use crate::{dns::DNSUpdater, ip::IPFinder};
use clap::{Parser, ValueEnum};
use dns::DigitalOcean;

mod dns;
mod ip;

pub type Address = std::net::Ipv4Addr;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, name = "token")]
    token_file: PathBuf,

    #[clap(short, long)]
    name: String,

    #[clap(value_enum, short, long, default_value = "digital-ocean")]
    dns: DNSProvider,

    #[clap(value_enum, short, long, default_value = "ifconfig")]
    ip: IPProvider,
}

#[derive(ValueEnum, Clone)]
pub enum DNSProvider {
    DigitalOcean,
}

#[derive(ValueEnum, Clone)]
pub enum IPProvider {
    Ifconfig,
}

fn main() -> eyre::Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    let token = std::fs::read_to_string(args.token_file)?.trim().to_string();
    let finder = args.ip.finder();
    let ip = finder.find_ip()?;
    let dns = args.dns.updater(token);
    dns.set_dns(&args.name, ip)?;

    Ok(())
}

impl DNSProvider {
    fn updater(&self, token: String) -> Box<dyn DNSUpdater> {
        match self {
            DNSProvider::DigitalOcean => Box::new(DigitalOcean::new(token)),
        }
    }
}

impl IPProvider {
    fn finder(&self) -> Box<dyn IPFinder> {
        match self {
            IPProvider::Ifconfig => Box::new(ip::IfConfig),
        }
    }
}
