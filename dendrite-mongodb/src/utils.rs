//! # MongoDB utilities
//!
//! Module `mongodb_search_utils` exports helper functions that facilitate storing Query Models in
//! MongoDB.

use anyhow::Result;
use dendrite_lib::axon_utils::TokenStore;
use log::{debug, warn};
use mongodb::{Client, Database};
use mongodb::bson::{Bson, doc, Document};
use mongodb::options::{ClientOptions,UpdateOptions};
use std::time;
use tokio::time::sleep;

/// Polls MongoDB until it is available and ready.
pub async fn wait_for_mongodb(url: &str, app_name: &str) -> Result<Client> {
    let interval = time::Duration::from_secs(1);
    loop {
        match try_to_connect(url, app_name).await {
            Err(e) => {
                warn!("Elastic Search is not ready (yet): {:?}", e);
            }
            Ok(client) => return Ok(client),
        }
        sleep(interval).await;
    }
}

async fn try_to_connect<S: Into<String>, T: Into<String>>(url: S, app_name: T) -> Result<Client> {
    let mut client_options = ClientOptions::parse(url.into()).await?;
    client_options.app_name = Some(app_name.into());
    let client = Client::with_options(client_options)?;
    let response = client.list_database_names(None, None).await?;

    for db_name in response {
        println!("MongoDB database: {:?}", db_name);
    }

    debug!("MongoDB: contacted");

    Ok(client)
}

#[derive(Clone)]
pub struct MongoQueryModel {
    database: Database,
    identifier: String,
}

impl MongoQueryModel {
    pub fn get_database(&self) -> &Database {
        &self.database
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

pub fn create_mongodb_query_model(
    mongo_client: Client,
    database_name: &str,
    identifier: String,
) -> MongoQueryModel {
    let database = mongo_client.database(database_name);
    MongoQueryModel {
        database,
        identifier,
    }
}

#[async_trait::async_trait]
impl TokenStore for MongoQueryModel {
    async fn store_token(&self, token: i64) {
        let identifier = &self.identifier;
        let hex_token = format!("{:x}", token);
        let mut options = UpdateOptions::default();
        options.upsert = Some(true);
        let result = self
            .database
            .collection::<Document>("tracking-token")
            .update_one(doc! {"_id": identifier}, doc!{"_id": identifier, "token": hex_token}, Some(options))
            .await;
        debug!(
            "MongoDB store token result: {:?}: {:?}",
            identifier, result
        );
    }

    async fn retrieve_token(&self) -> Result<i64> {
        let identifier = &self.identifier;
        let response = self
            .database
            .collection::<Document>("tracking-token")
            .find_one(doc! {"_id": identifier}, None)
            .await?;
        debug!("Retrieved response: {:?}: {:?}", identifier, response);
        if let Some(value) = response {
            let token = value.get("token");
            if let Some(Bson::String(hex_token)) = token {
                let token = i64::from_str_radix(hex_token, 16)?;
                debug!("Retrieved token: {:?}: {:?}", identifier, token);
                return Ok(token);
            }
        }
        Ok(-1)
    }
}
