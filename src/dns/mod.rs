use crate::Address;

mod r#do;
use clap::ValueEnum;
pub use r#do::DigitalOcean;

pub trait DNSUpdater {
    fn set_dns(&self, name: &str, addr: Address) -> eyre::Result<()>;
}

#[derive(ValueEnum, Clone)]
pub enum DNSProvider {
    DigitalOcean,
}

impl DNSProvider {
    pub fn updater(&self, token: String) -> Box<dyn DNSUpdater> {
        match self {
            DNSProvider::DigitalOcean => Box::new(DigitalOcean::new(token)),
        }
    }
}
