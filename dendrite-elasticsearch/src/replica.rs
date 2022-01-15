use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use elasticsearch::IndexParts;
use dendrite_lib::axon_server::event::Event;
use dendrite_lib::axon_utils::{AxonServerHandle, TheHandlerRegistry, TokenStore, empty_handler_registry, event_processor, HandlerRegistry, HandleBuilder};
use log::{debug, error, warn};
use serde_json::{json, Map, Value};
use serde::Serialize;
use dendrite_lib::intellij_work_around::Debuggable;
use crate::{ElasticQueryModel, create_elastic_query_model, wait_for_elastic_search};

pub(crate) type Transcoder = Arc<Box<dyn Fn(Bytes) -> anyhow::Result<Value> + Send + Sync>>;
pub(crate) type Deserializer<T> = dyn Fn(Bytes) -> Result<T, prost::DecodeError> + Send + Sync;
pub(crate) type DeserializerBox<T> = Box<Deserializer<T>>;

#[derive(Clone)]
pub struct Transcoders(HashMap<String,Transcoder>);

#[derive(Clone)]
struct ReplicaQueryModel {
    elastic_query_model: ElasticQueryModel,
    transcoders: Transcoders,
}

#[async_trait::async_trait]
impl TokenStore for ReplicaQueryModel {
    async fn store_token(&self, token: i64) {
        self.elastic_query_model.store_token(token).await;
    }

    async fn retrieve_token(&self) -> Result<i64> {
        self.elastic_query_model.retrieve_token().await
    }
}

pub fn process_events_with(transcoders: Transcoders) -> Box<dyn FnOnce(AxonServerHandle) -> Pin<Box<dyn Future<Output = ()> + Send>> + Sync> {
    Box::new(move |handle| Box::pin(process_events(handle, transcoders)))
}

/// Handles events for the `replica` query model.
///
/// Constructs an event handler registry and delegates to function `event_processor`.
pub async fn process_events(axon_server_handle: AxonServerHandle, transcoders: Transcoders) {
    if let Err(e) = internal_process_events(axon_server_handle, transcoders).await {
        error!("Error while handling events: {:?}", e);
    }
    debug!("Stopped handling events for replica query model");
}

async fn internal_process_events(axon_server_handle: AxonServerHandle, transcoders: Transcoders) -> Result<()> {
    let client = wait_for_elastic_search().await?;
    debug!("Elastic Search client: {:?}", client);

    let elastic_query_model = create_elastic_query_model(client, "replica".to_string());
    let query_model = ReplicaQueryModel{ elastic_query_model, transcoders };

    let mut event_handler_registry: TheHandlerRegistry<
        ReplicaQueryModel,
        Event,
        Option<ReplicaQueryModel>,
    > = empty_handler_registry();

    let handle = HandleBuilder::new(
        "any-event",
        &null_deserializer,
        &(|e,q,m| Box::pin(handle_registered_event(e, q, m)))
    ).ignore_output().build();
    event_handler_registry.append_category_handle(".*", handle)?;

    event_processor(axon_server_handle, query_model, event_handler_registry)
        .await
        .context("Error while handling commands")
}

fn null_deserializer(_buf: Bytes) -> Result<(),prost::DecodeError> {
    Ok(())
}

async fn handle_registered_event(
    _buf: (),
    event: Event,
    query_model: ReplicaQueryModel,
) -> Result<Option<()>> {
    let query_model_name = query_model.elastic_query_model.get_identifier().to_string();
    let event_id = event.message_identifier.clone();
    if let Err(e) = handle_registered_event_internal(event, query_model).await {
        warn!("Error processing event for query model: {:?}: {:?}: {:?}", event_id, query_model_name, e);
    }
    Ok(None)
}
async fn handle_registered_event_internal(
    event: Event,
    query_model: ReplicaQueryModel,
) -> Result<Option<()>> {
    debug!("Apply any event to ReplicaQueryModel: {:?}: {:?}", Debuggable::from(&event), query_model.elastic_query_model.get_identifier());
    if let Some(serialized_object) = &event.payload {
        let payload_type = &*serialized_object.r#type;
        let buf = &serialized_object.data;
        debug!("Replica: save {:?} ({:?})", payload_type, buf.len());

        let mut meta_data_json = Map::new();
        for (key, value) in event.meta_data.iter() {
            meta_data_json.insert(key.to_string(), serde_json::to_value(value)?);
        }
        let meta_data_json = Value::Object(meta_data_json);

        let mut json_value = Map::new();
        json_value.insert("type".to_string(), json!(payload_type));
        json_value.insert("event_id".to_string(), json!(event.message_identifier));
        json_value.insert("aggregate_id".to_string(), json!(event.aggregate_identifier));
        json_value.insert("aggregate_sequence_number".to_string(), json!(event.aggregate_sequence_number));
        json_value.insert("aggregate_type".to_string(), json!(event.aggregate_type));
        json_value.insert("timestamp".to_string(), json!(event.timestamp));
        json_value.insert("meta_data".to_string(), meta_data_json);
        if event.snapshot {
            json_value.insert("is_snapshot".to_string(), json!(event.snapshot));
        }
        if let Some(transcoder) = query_model.transcoders.0.get(payload_type) {
            let event_json_result: Result<Value> = (transcoder)(Bytes::from(buf.clone()));
            if let Ok(event_json) = event_json_result {
                debug!("Replica: event JSON: {:?}", event_json);
                json_value.insert("event".to_string(), event_json);
            }
        };

        let json_value = Value::Object(json_value);
        debug!("Replica: JSON value: {:?}", json_value);

        let es_client = query_model.elastic_query_model.get_client();
        debug!("Replica: ElasticSearch client: {:?}", es_client);
        let response = es_client
            .index(IndexParts::IndexId(query_model.elastic_query_model.get_identifier(),&*event.message_identifier))
            .body(json_value)
            .send()
            .await;
        debug!("Elastic Search response: {:?}", response);
    }
    Ok(None)
}

impl Transcoders {
    pub fn new() -> Self {
        Transcoders(HashMap::new())
    }

    pub fn insert_ref<T: 'static + Serialize>(self, message_type: &str, deserializer: &'static Deserializer<T>) -> Transcoders {
        self.insert(message_type, Box::new(deserializer))
    }
    pub fn insert<T: 'static + Serialize>(mut self, message_type: &str, deserializer: DeserializerBox<T>) -> Transcoders {
        let transcoder = move |buf: Bytes| {
            let object: T = (deserializer)(buf).map_err(|e| anyhow!("Deserialize protobuf error: {:?}", e))?;
            let result: anyhow::Result<Value> = serde_json::to_value(object).map_err(|e| anyhow!("Serialize JSON error: {:?}", e));
            result
        };
        self.0.insert(message_type.to_string(), Arc::new(Box::new(transcoder)));
        self
    }
    pub fn insert_transcoder(mut self, message_type: &str, transcoder: &'static (dyn Fn(Bytes) -> anyhow::Result<Value> + Send + Sync)) -> Transcoders {
        self.0.insert(message_type.to_string(), Arc::new(Box::new(transcoder)));
        self
    }
}
