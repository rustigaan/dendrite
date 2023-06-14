//! # MongoDB utilities
//!
//! Module `mongodb_search_utils` exports helper functions that facilitate storing Query Models in
//! MongoDB.

use anyhow::Result;
use dendrite_lib::axon_utils::TokenStore;
use log::{debug, warn};
use mongodb::bson::{doc, Bson, Document};
use mongodb::options::{ClientOptions, ReplaceOptions};
use mongodb::{Client, Collection, Database};
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
    identifier: String,
    database: Database,
}

impl MongoQueryModel {
    pub fn get_database(&self) -> &Database {
        &self.database
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

pub fn create_mongodb_query_model<E: Into<String>>(
    identifier: E,
    mongo_client: Client,
    database_name: &str,
) -> MongoQueryModel {
    let identifier = identifier.into();
    let database = mongo_client.database(database_name);
    MongoQueryModel {
        identifier,
        database,
    }
}

#[async_trait::async_trait]
impl TokenStore for MongoQueryModel {
    async fn store_token(&self, token: i64) {
        let identifier = &self.identifier;
        let hex_token = format!("{:x}", token);
        let mut options = ReplaceOptions::default();
        options.upsert = Some(true);
        let result = self
            .database
            .collection::<Document>("tracking-token")
            .replace_one(
                doc! {"_id": identifier},
                doc! {"_id": identifier, "token": hex_token},
                Some(options),
            )
            .await;
        debug!("MongoDB store token result: {:?}: {:?}", identifier, result);
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

#[derive(Clone)]
pub struct MongoCollectionQueryModel {
    mongo_query_model: MongoQueryModel,
    collection: Collection<Document>,
}

impl MongoCollectionQueryModel {
    pub fn get_database(&self) -> &Database {
        &self.mongo_query_model.database
    }

    pub fn get_identifier(&self) -> &str {
        &self.mongo_query_model.identifier
    }

    pub fn get_collection(&self) -> &Collection<Document> {
        &self.collection
    }
}

pub fn create_mongodb_collection_query_model<E: Into<String>>(
    identifier: E,
    mongo_client: Client,
    database_name: &str,
    collection_name: &str,
) -> MongoCollectionQueryModel {
    let identifier = identifier.into();
    let database = mongo_client.database(database_name);
    let collection = database.collection(collection_name);
    let mongo_query_model = MongoQueryModel {
        database,
        identifier,
    };
    MongoCollectionQueryModel {
        mongo_query_model,
        collection,
    }
}

#[async_trait::async_trait]
impl TokenStore for MongoCollectionQueryModel {
    async fn store_token(&self, token: i64) {
        self.mongo_query_model.store_token(token).await
    }

    async fn retrieve_token(&self) -> Result<i64> {
        self.mongo_query_model.retrieve_token().await
    }
}
