use hex::{ToHex, FromHex};
use std::str;
use anyhow::{anyhow, Error};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize};
use url::{Url, ParseError};


pub struct ZClient {
    c: Client,
    pub url: String,
    pub user: String,
    pub password: Option<String>,
}

impl Default for ZClient {
    fn default() -> Self {
        Self {
            c: Client::default(),
            url: "http://127.0.0.1:9999".to_string(),
            user: String::new(),
            password: None,
        }
    }
}

pub struct ZClientBuilder {
    client: ZClient,
}

impl Default for ZClientBuilder {
    fn default() -> Self {
        ZClientBuilder {
            client: ZClient::default(),
        }
    }
}

impl ZClientBuilder {
    pub fn with_url(mut self, url: String) -> Result<Self, ParseError> {
        let _ = Url::parse(&url)?;
        self.client.url = url;
        Ok(self)
    }

    pub fn with_auth(mut self, user: String, password: Option<String>) -> Self {
        self.client.user = user;
        self.client.password = password;
        self
    }

    pub fn build(self) -> ZClient {
        self.client
    }
}

#[derive(Deserialize)]
pub struct BalanceResponse {
    result: f32,
    error: Option<String>,
    id: Option<i32>,
}

#[derive(Deserialize)]
pub struct ListAddressesResponse {
    result: Vec<String>,
    error: Option<String>,
    id: Option<i32>,
}

impl ZClient {
    pub fn builder() -> ZClientBuilder {
        ZClientBuilder::default()
    }

    pub fn getbalance(&self) -> Result<f32, Error> {
        let res = self.c.get(self.url.clone())
            .basic_auth(self.user.clone(), self.password.clone())
            .header(CONTENT_TYPE, "text/octet-stream")
            .body("{\"jsonrpc\": \"1.0\", \"method\": \"getbalance\", \"params\": []}")
            .send()?
            .json::<BalanceResponse>()?;
        Ok(res.result)
    }

    pub fn z_listaddresses(&self) -> Result<Vec<String>, Error> {
        let res = self.c.get(self.url.clone())
            .basic_auth(self.user.clone(), self.password.clone())
            .header(CONTENT_TYPE, "text/octet-stream")
            .body("{\"jsonrpc\": \"1.0\", \"method\": \"z_listaddresses\", \"params\": []}")
            .send()?
            .json::<ListAddressesResponse>()?;
        Ok(res.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use httpmock::Method::GET;
    use serde_json::{json, Value::Null};
    
    #[test]
    fn test_getbalance() {
        let server = MockServer::start();

        let getbalance_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/getbalance");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(json!({
                    "result": 2.5,
                    "error": Null,
                    "id": Null,
                }).to_string());
        });

        let balance = ZClient::builder()
            .with_url(server.url("/getbalance"))
            .expect("Failed to parse URL")
            .with_auth("user".to_string(), Some("pass".to_string()))
            .build()
            .getbalance()
            .expect("Failed to build client");

        assert!(balance == 2.5);
    }

    #[test]
    fn test_z_listaddresses() {
        let server = MockServer::start();

        let listaddresses_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/z_listaddresses");
            then.status(200)
                .header("Content-Type", "application/json")
                .body(json!({
                    "result": vec!["z_addr1", "z_addr2", "z_addr3"],
                    "error": Null,
                    "id": Null,
                }).to_string());
        });

        let addresses = ZClient::builder()
            .with_url(server.url("/z_listaddresses"))
            .expect("Failed to parse URL")
            .with_auth("user".to_string(), Some("pass".to_string()))
            .build()
            .z_listaddresses()
            .expect("Failed to build client");

        assert!(addresses == vec![
            "z_addr1".to_string(), 
            "z_addr2".to_string(), 
            "z_addr3".to_string(),
        ]);
    }
}

