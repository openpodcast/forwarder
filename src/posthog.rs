use std::collections::HashMap;

use serde::Serialize;
use worker::wasm_bindgen::JsValue;
use worker::{Fetch, Method, Request, RequestInit, Result};

extern crate serde_json;

const API_ENDPOINT: &str = "https://app.posthog.com/capture/";

pub struct ClientConfig {
    api_endpoint: String,
    api_key: String,
}

impl From<&str> for ClientConfig {
    fn from(api_key: &str) -> Self {
        Self {
            api_endpoint: API_ENDPOINT.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

impl From<String> for ClientConfig {
    fn from(api_key: String) -> Self {
        Self::from(api_key.as_str())
    }
}

pub struct Client {
    config: ClientConfig,
}

impl Client {
    pub fn new<C: Into<ClientConfig>>(config: C) -> Self {
        Self {
            config: config.into(),
        }
    }

    pub async fn send(&self, event: Event) -> Result<worker::Response> {
        let inner_event = InnerEvent::new(event, self.config.api_key.clone());
        let r = Request::new_with_init(
            &self.config.api_endpoint,
            &RequestInit {
                method: Method::Post,
                body: Some(JsValue::from_str(&serde_json::to_string(&inner_event)?)),
                ..RequestInit::default()
            },
        )?;
        Fetch::Request(r).send().await
    }
}

// This exists so that the client doesn't have to specify the API key over and
// over
#[derive(Serialize)]
struct InnerEvent {
    api_key: String,
    event: String,
    properties: Properties,
}

impl InnerEvent {
    // That's a false-positive
    #[allow(clippy::missing_const_for_fn)]
    fn new(event: Event, api_key: String) -> Self {
        Self {
            api_key,
            event: event.event,
            properties: event.properties,
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Event {
    event: String,
    properties: Properties,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Properties {
    distinct_id: String,
    #[serde(flatten)]
    props: HashMap<String, serde_json::Value>,
}

impl Properties {
    fn new<S: Into<String>>(distinct_id: S) -> Self {
        Self {
            distinct_id: distinct_id.into(),
            props: HashMap::default(),
        }
    }
}

impl Event {
    pub fn new<S: Into<String>>(event: S, distinct_id: S) -> Self {
        Self {
            event: event.into(),
            properties: Properties::new(distinct_id),
        }
    }

    /// Errors if `prop` fails to serialize
    pub fn property<K: Into<String>, P: Serialize>(mut self, key: K, prop: P) -> Result<Self> {
        let as_json = serde_json::to_value(prop).map_err(worker::Error::SerdeJsonError)?;
        self.properties.props.insert(key.into(), as_json);
        Ok(self)
    }
}

#[cfg(test)]
pub mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_create_event() -> Result<()> {
        let expected = json!({
            "api_key": "api_key",
            "event": "rss_feed_request",
            "properties": {
                "distinct_id": "user1",
                "client": "Podcast Client",
                "url": "https://www.example.com",
            },
        });

        let event = Event::new("rss_feed_request", "user1")
            .property("client", "Podcast Client")?
            .property("url", "https://www.example.com")?;

        let inner_event = InnerEvent::new(event, "api_key".to_string());

        assert_eq!(expected, serde_json::to_value(inner_event).unwrap());

        // Async tests don't work yet
        // let client = Client::new("api_key");
        // client.send(event).await.unwrap();
        Ok(())
    }

    #[test]
    fn test_child_map() {
        let event = Event::new("rss_feed_request", "user1")
            .property("client", "Podcast Client")
            .unwrap();
        let mut child_map = HashMap::new();
        child_map.insert("child_key1", "child_value1");
        event.property("key3", child_map).unwrap();
    }
}
