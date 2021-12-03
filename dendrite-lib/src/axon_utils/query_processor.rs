use super::handler_registry::TheHandlerRegistry;
use crate::axon_server::query::query_service_client::QueryServiceClient;
use crate::axon_server::query::{
    query_provider_inbound, query_provider_outbound, QueryProviderOutbound,
};
use crate::axon_server::query::{QueryComplete, QueryRequest, QueryResponse, QuerySubscription};
use crate::axon_server::{FlowControl, SerializedObject};
use crate::axon_utils::AxonServerHandle;
use crate::intellij_work_around::Debuggable;
use anyhow::{anyhow, Result};
use async_stream::stream;
use futures_core::stream::Stream;
use log::{debug, error, warn};
use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tonic::Request;
use uuid::Uuid;

/// Marker trait that describes the context for a query handler.
pub trait QueryContext {}

/// Carries the result of a query from handler to processor.
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub payload: Option<SerializedObject>,
}

/// Carries the result of a query from the query processor th the output stream.
#[derive(Debug)]
struct AxonQueryResult {
    message_identifier: String,
    result: Option<SerializedObject>,
}

/// Subscribes to queries, executes them against a query model and pass back the results.
pub async fn query_processor<Q: QueryContext + Send + Sync + Clone>(
    axon_server_handle: AxonServerHandle,
    query_context: Q,
    query_handler_registry: TheHandlerRegistry<Q, QueryRequest, QueryResult>,
) -> Result<()> {
    debug!("Query processor: start");

    let mut client = QueryServiceClient::new(axon_server_handle.conn.clone());

    let query_vec = query_handler_registry.handlers.keys().cloned().collect();

    let (tx, rx): (Sender<AxonQueryResult>, Receiver<AxonQueryResult>) = channel(10);

    let outbound = create_output_stream(axon_server_handle, query_vec, rx);

    debug!("Query processor: calling open_stream");
    let response = client.open_stream(Request::new(outbound)).await?;
    debug!("Stream response: {:?}", response);

    let mut inbound = response.into_inner();
    loop {
        match inbound.message().await {
            Ok(Some(inbound)) => {
                debug!("Inbound message: {:?}", Debuggable::from(&inbound));
                if let Some(query_provider_inbound::Request::Query(query)) = inbound.request {
                    let query_name = query.query.clone();
                    let mut result = Err(anyhow!("Could not find aggregate handler"));
                    if let Some(query_handle) = query_handler_registry.handlers.get(&query_name) {
                        if let QueryRequest {
                            payload: Some(serialized_object),
                            ..
                        } = &query
                        {
                            result = query_handle
                                .handle(
                                    serialized_object.data.clone(),
                                    query.clone(),
                                    query_context.clone(),
                                )
                                .await
                        }
                    }

                    match result.as_ref() {
                        Err(e) => warn!("Error while handling query: {:?}", e),
                        Ok(Some(result)) => debug!("Result from query handler: {:?}", result),
                        Ok(None) => debug!("Result from query handler: None"),
                    }

                    let axon_query_result = AxonQueryResult {
                        message_identifier: query.message_identifier,
                        result: result
                            .unwrap_or(None)
                            .map(|query_result| query_result.payload)
                            .flatten(),
                    };
                    tx.send(axon_query_result).await.unwrap();
                }
            }
            Ok(None) => {
                debug!("None incoming");
            }
            Err(e) => {
                error!("Error from AxonServer: {:?}", e);
                return Err(anyhow!(e.code()));
            }
        }
    }
}

fn create_output_stream(
    axon_server_handle: AxonServerHandle,
    query: Vec<String>,
    mut rx: Receiver<AxonQueryResult>,
) -> impl Stream<Item = QueryProviderOutbound> {
    stream! {
        let client_id = axon_server_handle.client_id.clone();
        debug!("Query processor: stream: start: {:?}", rx);
        for query_name in query.iter() {
            debug!("Query processor: stream: subscribe to query type: {:?}", query_name);
            let subscription_id = Uuid::new_v4();
            let subscription = QuerySubscription {
                message_id: format!("{}", subscription_id),
                query: query_name.to_string().clone(),
                result_name: "*".to_string(),
                client_id: client_id.clone(),
                component_name: axon_server_handle.display_name.clone(),
            };
            debug!("Subscribe query: Subscription: {:?}", subscription);
            let instruction_id = Uuid::new_v4();
            debug!("Subscribe query: Instruction ID: {:?}", instruction_id);
            let instruction = QueryProviderOutbound {
                instruction_id: format!("{}", instruction_id),
                request: Some(query_provider_outbound::Request::Subscribe(subscription)),
            };
            yield instruction.to_owned();
        }

        let permits_batch_size: i64 = 3;
        let mut permits = permits_batch_size * 2;
        debug!("Query processor: stream: send initial flow-control permits: amount: {:?}", permits);
        let flow_control = FlowControl {
            client_id: client_id.clone(),
            permits,
        };
        let instruction_id = Uuid::new_v4();
        let instruction = QueryProviderOutbound {
            instruction_id: format!("{}", instruction_id),
            request: Some(query_provider_outbound::Request::FlowControl(flow_control)),
        };
        yield instruction.to_owned();

        while let Some(axon_query_result) = rx.recv().await {
            debug!("Send query response: {:?}", axon_query_result);
            let response_id = Uuid::new_v4();
            let response = QueryResponse {
                message_identifier: format!("{}", response_id),
                error_code: "".to_string(),
                error_message: None,
                payload: axon_query_result.result.clone(),
                meta_data: HashMap::new(),
                processing_instructions: Vec::new(),
                request_identifier: axon_query_result.message_identifier.clone(),
            };
            let instruction_id = Uuid::new_v4();
            let instruction = QueryProviderOutbound {
                instruction_id: format!("{}", instruction_id),
                request: Some(query_provider_outbound::Request::QueryResponse(response)),
            };
            debug!("QueryResponse instruction: {:?}", instruction);
            yield instruction.to_owned();

            let complete_id = Uuid::new_v4();
            let complete = QueryComplete {
                message_id: format!("{}", complete_id),
                request_id: axon_query_result.message_identifier.clone(),
            };
            let complete_instruction_id = Uuid::new_v4();
            let complete_instruction = QueryProviderOutbound {
                instruction_id: format!("{}", complete_instruction_id),
                request: Some(query_provider_outbound::Request::QueryComplete(complete)),
            };
            debug!("Complete instruction: {:?}", complete_instruction);
            yield complete_instruction.to_owned();

            permits -= 1;
            if permits <= permits_batch_size {
                debug!("Query processor: stream: send more flow-control permits: amount: {:?}", permits_batch_size);
                let flow_control = FlowControl {
                    client_id: client_id.clone(),
                    permits: permits_batch_size,
                };
                let instruction_id = Uuid::new_v4();
                let instruction = QueryProviderOutbound {
                    instruction_id: format!("{}", instruction_id),
                    request: Some(query_provider_outbound::Request::FlowControl(flow_control)),
                };
                yield instruction.to_owned();
                permits += permits_batch_size;
            }
            debug!("Query processor: stream: flow-control permits: balance: {:?}", permits);
        }

        // debug!("Query processor: stream: stop");
    }
}
