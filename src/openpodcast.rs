//! Client for the Open Podcast API (<https://openpodcast.dev>)
//!
//! # Example
//!
//! ```rust
//! use openpodcast::Client;
//!
//! let client = Client::new("token");
//! let json = serde_json::json!({
//!    "ip": "127.0.0.1",
//!    "user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36",
//! });
//! let response = client.send(json).await?;
//! assert_eq!(response.status(), 200);
//! ```
//!
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use worker::Result;

extern crate serde_json;

pub struct Client {
    endpoint: String,
    token: String,
}

impl Client {
    /// Create a new client
    pub fn new(endpoint: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            token: token.into(),
        }
    }

    /// Send a request to the API
    pub async fn send(&self, data: serde_json::Value) -> Result<reqwest::Response> {
        // let data = JsValue::from_serde(&data)?;
        // console_log!("data: {:?}", data);
        // convert [Object object]

        // let r = Request::new_with_init(
        //     &self.endpoint,
        //     &RequestInit {
        //         method: Method::Post,
        //         headers: {
        //             let mut headers = worker::Headers::new();
        //             headers.set("Authorization", &format!("Bearer {}", self.token))?;
        //             headers.set("Content-Type", "application/json")?;
        //             headers
        //         },
        //         body: Some(data),
        //         ..RequestInit::default()
        //     },
        // )?;

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))
                .map_err(|e| worker::Error::from(e.to_string()))?,
        );
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let response = client
            .post(&self.endpoint)
            .headers(headers)
            .json(&data)
            .send()
            .await
            .map_err(|e| worker::Error::from(e.to_string()))?;

        Ok(response)
    }
}
