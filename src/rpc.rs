use hex::{ToHex, FromHex};
use std::str;
use anyhow::{anyhow, Error};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use url::{Url, ParseError};
use serde_json::{self, json};


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

#[derive(Debug, Deserialize)]
pub struct ZResponse<T> {
    result: T,
    error: Option<String>,
    id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ZRequest<T> {
    jsonrpc: String,
    method: String,
    params: Vec<T>,
}

pub struct ZRequestBuilder<T> {
    request: ZRequest<T>,
}

impl<T> Default for ZRequestBuilder<T> {
    fn default() -> Self {
        Self {
            request: ZRequest::default(),
        }
    }
}

impl<T> ZRequestBuilder<T> {
    pub fn jsonrpc(mut self, version: String) -> Self {
        self.request.jsonrpc = version;
        self
    }

    pub fn method(mut self, method: String) -> Self {
        self.request.method = method;
        self
    }

    pub fn params(mut self, params: Vec<T>) -> Self {
        self.request.params = params;
        self
    }

    pub fn build(self) -> ZRequest<T> {
        self.request
    }
}

impl<T> Default for ZRequest<T> {
    fn default() -> Self {
        Self {
            jsonrpc: "1.0".to_string(),
            method: "getbalance".to_string(),
            params: vec![],
        }
    }
}

impl<T> ZRequest<T> {
    pub fn builder() -> ZRequestBuilder<T> {
        ZRequestBuilder::<T>::default()
    }
}

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

impl ZClient {
    pub fn builder() -> ZClientBuilder {
        ZClientBuilder::default()
    }

    fn send<S, T>(&self, req: ZRequest<S>) -> Result<ZResponse<T>, Error> 
    where S: Serialize + 'static, T: DeserializeOwned + 'static {
        let res = self.c.post(self.url.clone())
            .basic_auth(self.user.clone(), self.password.clone())
            .header(CONTENT_TYPE, "text/octet-stream")
            .body(json!(req).to_string())
            .send()?
            .json::<ZResponse<T>>()?;
        Ok(res)
    }
    
    /* Zcash RPC API implementation */

    pub fn getbalance(&self) -> Result<f32, Error> {
        let req = ZRequest::<String>::builder()
            .method("getbalance".to_string())
            .build();
            
        let res: ZResponse<f32> = self.send::<String, f32>(req)?;
        Ok(res.result)
    }

    pub fn z_listaddresses(&self) -> Result<Vec<String>, Error> {
        let res: ZResponse<Vec<String>> = self.send::<String, Vec<String>>(ZRequest {
            jsonrpc: "1.0".to_string(),
            method: "z_listaddresses".to_string(),
            params: vec![],
        })?;
        Ok(res.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use httpmock::Method::POST;
    use serde_json::{json, Value::Null};
    
    #[test]
    fn test_getbalance() {
        let server = MockServer::start();

        let getbalance_mock = server.mock(|when, then| {
            when.method(POST)
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
            when.method(POST)
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

