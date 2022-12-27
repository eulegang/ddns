use crate::Address;

mod ifconfig;

pub use ifconfig::IfConfig;

pub trait IPFinder {
    fn find_ip(&self) -> eyre::Result<Address>;
}
