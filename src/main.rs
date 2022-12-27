use std::path::PathBuf;

use clap::Parser;
use dns::DNSProvider;
use ip::IPProvider;

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
