use crate::Address;

mod ifconfig;

use clap::ValueEnum;
pub use ifconfig::IfConfig;

pub trait IPFinder {
    fn find_ip(&self) -> eyre::Result<Address>;
}

#[derive(ValueEnum, Clone)]
pub enum IPProvider {
    Ifconfig,
}

impl IPProvider {
    pub fn finder(&self) -> Box<dyn IPFinder> {
        match self {
            IPProvider::Ifconfig => Box::new(IfConfig),
        }
    }
}
