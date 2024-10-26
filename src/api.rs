use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

const URL: &str = "https://api.shodan.io";
const URL_DOMAIN: &str = "https://api.shodan.io/dns/domain/";

#[derive(Deserialize, Debug)]
pub struct UsageLimits {
    pub scan_credits: i32,
    pub query_credits: i32,
    pub monitored_ips: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct JsonData {
    pub scan_credits: i32,
    pub usage_limits: UsageLimits,
    pub plan: String,
    pub https: bool,
    pub unlocked: bool,
    pub query_credits: i32,
    pub monitored_ips: Option<i32>,
    pub unlocked_left: i32,
    pub telnet: bool,
}

#[derive(Deserialize, Debug)]
pub struct JsonSubDomain {
    pub subdomains: Option<Vec<String>>,
}

pub struct API {
    api_key: String,
    client: Client,
}

impl API {
    pub fn new(api_key: String) -> Self {
        API {
            api_key,
            client: Client::new(),
        }
    }

    pub fn set_api_key(&mut self, new_key: String) {
        self.api_key = new_key;
    }

    pub async fn info_account(&self, verbose: bool) -> Result<JsonData, Box<dyn Error>> {
        let url = format!("{}/api-info?key={}", URL, self.api_key);
        if verbose {
            println!("Checking API credits...");
        }

        let response = self.client.get(&url).send().await?;
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err("Unauthorized access - invalid API key".into());
        }

        let data: JsonData = response.json().await?;
        if verbose {
            println!(
                "API Plan: {}, Query Credits: {}",
                data.plan, data.query_credits
            );
        }

        if data.query_credits <= 0 {
            return Err("No query credits available on the account".into());
        }

        Ok(data)
    }

    pub async fn get_subdomain(&self, domain: &str, verbose: bool) -> Result<JsonSubDomain, Box<dyn Error>> {
        let url = format!("{}{}?key={}", URL_DOMAIN, domain, self.api_key);
        if verbose {
            println!("Sending request to Shodan API for subdomains");
        }

        let response = self.client.get(&url).send().await?;
        let data: JsonSubDomain = response.json().await?;
        Ok(data)
    }
}
