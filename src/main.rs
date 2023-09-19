use std::path::PathBuf;

use clap::Parser;
use dns::DNSProvider;
use ip::IPProvider;

mod dns;
mod ip;

pub type Address = std::net::Ipv4Addr;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long, name = "token", env = "TOKEN")]
    token_file: PathBuf,

    #[clap(short, long, name = "name")]
    names: Vec<String>,

    #[clap(
        value_enum,
        short,
        long,
        default_value = "digital-ocean",
        env = "DNSPROVIDER"
    )]
    dns: DNSProvider,

    #[clap(
        value_enum,
        short,
        long,
        default_value = "ifconfig",
        env = "ENVPROVIDER"
    )]
    ip: IPProvider,

    #[clap(short, long, env = "WAIT")]
    wait: Option<u64>,
}

fn main() -> eyre::Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    if let Some(wait) = args.wait {
        loop {
            let token = std::fs::read_to_string(&args.token_file)?
                .trim()
                .to_string();
            let finder = args.ip.finder();
            let ip = finder.find_ip()?;
            let dns = args.dns.updater(token);

            for name in &args.names {
                dns.set_dns(&name, ip)?;
            }

            std::thread::sleep(std::time::Duration::from_secs(60 * wait))
        }
    } else {
        let token = std::fs::read_to_string(args.token_file)?.trim().to_string();
        let finder = args.ip.finder();
        let ip = finder.find_ip()?;
        let dns = args.dns.updater(token);

        for name in &args.names {
            dns.set_dns(&name, ip)?;
        }
    }

    Ok(())
}
