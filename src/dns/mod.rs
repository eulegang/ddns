use crate::Address;

mod r#do;
pub use r#do::DigitalOcean;

pub trait DNSUpdater {
    fn set_dns(&self, name: &str, addr: Address) -> eyre::Result<()>;
}
