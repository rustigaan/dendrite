use super::{AxonServerHandle, QuerySink, VecU8Message};
use crate::axon_server::query::query_service_client::QueryServiceClient;
use crate::axon_server::query::{QueryRequest, QueryResponse};
use crate::axon_server::SerializedObject;
use crate::intellij_work_around::Debuggable;
use anyhow::Result;
use log::debug;
use std::collections::HashMap;
use std::vec::Vec;
use uuid::Uuid;

#[tonic::async_trait]
impl QuerySink for AxonServerHandle {
    async fn send_query<'a>(
        &self,
        query_type: &str,
        query: &(dyn VecU8Message + Sync),
    ) -> Result<Vec<SerializedObject>> {
        debug!("Sending query: {:?}: {:?}", query_type, self.display_name);
        let mut buf = Vec::new();
        query.encode_u8(&mut buf).unwrap();
        let buffer_length = buf.len();
        debug!("Buffer length: {:?}", buffer_length);
        let serialized_command = SerializedObject {
            r#type: query_type.to_string(),
            revision: "1".to_string(),
            data: buf,
        };
        submit_query(self, &serialized_command).await
    }
}

async fn submit_query<'a>(
    this: &AxonServerHandle,
    message: &SerializedObject,
) -> Result<Vec<SerializedObject>> {
    debug!("Message: {:?}", Debuggable::from(message));
    let this = this.clone();
    let conn = this.conn;
    let mut client = QueryServiceClient::new(conn);
    debug!("Query Service Client: {:?}", client);
    let uuid = Uuid::new_v4();
    let query_request = QueryRequest {
        message_identifier: format!("{}", uuid),
        query: message.r#type.clone(),
        response_type: None,
        payload: Some(message.clone()),
        client_id: this.client_id.clone(),
        component_name: this.display_name.clone(),
        meta_data: HashMap::new(),
        processing_instructions: Vec::new(),
        timestamp: 0,
    };
    let response = client.query(query_request).await?;
    debug!("Response: {:?}", response);
    let mut response = response.into_inner();

    let mut result = Vec::new();
    loop {
        let query_response = response.message().await?;

        if let Some(QueryResponse {
            payload: Some(payload),
            ..
        }) = query_response
        {
            let payload = payload.clone();
            debug!("Query response: payload: {:?}", Debuggable::from(&payload));
            result.push(payload);
        } else {
            break;
        }
    }
    Ok(result)
}
