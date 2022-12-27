#![allow(dead_code)]

use reqwest::blocking::Client;

use super::DNSUpdater;

pub struct DigitalOcean {
    token: String,
}

impl DigitalOcean {
    pub fn new(token: String) -> DigitalOcean {
        DigitalOcean { token }
    }
}

impl DNSUpdater for DigitalOcean {
    fn set_dns(&self, name: &str, addr: crate::Address) -> eyre::Result<()> {
        let client = Client::default();
        let Some((tip, base)) = name.split_once('.') else {
            eyre::bail!("{name} is not a dns name")
        };

        let res = client
            .get(format!(
                "https://api.digitalocean.com/v2/domains/{base}/records"
            ))
            .bearer_auth(&self.token)
            .header("Accept", "application/json")
            .query(&[("name", &name)])
            .send()?
            .error_for_status()?;

        let res: DomainList = res.json()?;

        let id = res.domain_records.first().map(|r| r.id);

        if let Some(id) = id {
            let update = DomainUpdate {
                ty: None,
                name: None,
                data: addr.to_string(),
            };

            client
                .patch(format!(
                    "https://api.digitalocean.com/v2/domains/{base}/records/{id}"
                ))
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .json(&update)
                .send()?
                .error_for_status()?;
        } else {
            let update = DomainUpdate {
                ty: Some("A".to_string()),
                name: Some(tip.to_string()),
                data: addr.to_string(),
            };

            client
                .post(format!(
                    "https://api.digitalocean.com/v2/domains/{base}/records"
                ))
                .bearer_auth(&self.token)
                .header("Content-Type", "application/json")
                .json(&update)
                .send()?
                .error_for_status()?;
        }

        Ok(())
    }
}

impl DigitalOcean {
    fn build_client(&self) -> eyre::Result<reqwest::blocking::Client> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let mut bearer =
            reqwest::header::HeaderValue::from_str(&format!("Bearer: {}", dbg!(&self.token)))?;
        bearer.set_sensitive(true);
        headers.insert("Authorization", bearer);

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(client)
    }
}

#[derive(serde::Deserialize, Debug)]
struct DomainList {
    domain_records: Vec<DomainRecord>,
}

#[derive(serde::Deserialize, Debug)]
struct DomainRecord {
    #[serde(rename = "type")]
    ty: String,

    id: u64,
    name: String,
    data: String,
}

#[derive(serde::Serialize, Debug)]
struct DomainUpdate {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    ty: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    data: String,
}
