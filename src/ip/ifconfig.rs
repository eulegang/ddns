use super::IPFinder;
use crate::Address;

pub struct IfConfig;

impl IPFinder for IfConfig {
    fn find_ip(&self) -> eyre::Result<Address> {
        let res = reqwest::blocking::get("https://ifconfig.me/")?.text()?;
        let addr: Address = res.parse()?;

        Ok(addr)
    }
}
