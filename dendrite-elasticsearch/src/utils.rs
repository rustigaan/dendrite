//! # Elastic Search utilities
//!
//! Module `elastic_search_utils` exports helper functions that facilitate storing Query Models in
//! Elastic Search.

use anyhow::Result;
use dendrite_lib::axon_utils::TokenStore;
use elasticsearch::cluster::ClusterStatsParts;
use elasticsearch::http::transport::Transport;
use elasticsearch::{Elasticsearch, GetParts, IndexParts};
use log::{debug, warn};
use serde_json::{json, Value};
use std::time;
use tokio::time::sleep;

/// Polls ElasticSearch until it is available and ready.
pub async fn wait_for_elastic_search() -> Result<Elasticsearch> {
    let interval = time::Duration::from_secs(1);
    loop {
        match try_to_connect().await {
            Err(e) => {
                warn!("Elastic Search is not ready (yet): {:?}", e);
            }
            Ok(client) => return Ok(client),
        }
        sleep(interval).await;
    }
}

async fn try_to_connect() -> Result<Elasticsearch> {
    let transport = Transport::single_node("http://elastic-search:9200")?;
    let client = Elasticsearch::new(transport);
    let response = client.info().send().await?;

    let response_body = response.json::<Value>().await?;
    debug!("Info response body: {:?}", response_body);

    wait_for_status_ready(&client).await?;

    debug!("Elastic Search: contacted");

    Ok(client)
}

async fn wait_for_status_ready(client: &Elasticsearch) -> Result<()> {
    let interval = time::Duration::from_secs(1);
    loop {
        let response = client
            .cluster()
            .stats(ClusterStatsParts::NodeId(&["*"]))
            .send()
            .await?;
        let response_body = response.json::<Value>().await?;
        let status = response_body.as_object().map(|o| o.get("status")).flatten();
        debug!("Elastic Search status: {:?}", status);
        if let Some(Value::String(status_code)) = status {
            if *status_code != "red" {
                return Ok(());
            }
        }
        sleep(interval).await;
    }
}

#[derive(Clone)]
pub struct ElasticQueryModel {
    es_client: Elasticsearch,
    identifier: String,
}

impl ElasticQueryModel {
    pub fn get_client(&self) -> &Elasticsearch {
        &self.es_client
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

pub fn create_elastic_query_model(
    es_client: Elasticsearch,
    identifier: String,
) -> ElasticQueryModel {
    ElasticQueryModel {
        es_client,
        identifier,
    }
}

#[async_trait::async_trait]
impl TokenStore for ElasticQueryModel {
    async fn store_token(&self, token: i64) {
        let identifier = &self.identifier;
        let hex_token = format!("{:x}", token);
        let result = self
            .es_client
            .index(IndexParts::IndexId("tracking-token", identifier))
            .body(json!({
                "id": identifier.clone(),
                "token": hex_token,
            }))
            .send()
            .await;
        debug!(
            "Elastic Search store token result: {:?}: {:?}",
            identifier, result
        );
    }

    async fn retrieve_token(&self) -> Result<i64> {
        let identifier = &self.identifier;
        let response = self
            .es_client
            .get(GetParts::IndexId("tracking-token", identifier))
            ._source(&["token"])
            .send()
            .await?;
        let value = response.json::<Value>().await?;
        debug!("Retrieved response value: {:?}: {:?}", identifier, value);
        if let Value::String(hex_token) = &value["_source"]["token"] {
            let token = i64::from_str_radix(hex_token, 16)?;
            debug!("Retrieved token: {:?}: {:?}", identifier, token);
            return Ok(token);
        }
        Ok(-1)
    }
}
