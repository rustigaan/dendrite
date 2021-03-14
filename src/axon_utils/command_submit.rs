use anyhow::{Result,anyhow};
use log::{debug};
use std::collections::HashMap;
use std::vec::Vec;
use uuid::Uuid;
use super::{CommandSink, AxonServerHandle, wait_for_server, VecU8Message};
use crate::axon_server::SerializedObject;
use crate::axon_server::command::Command;
use crate::axon_server::command::command_service_client::CommandServiceClient;

/// Polls AxonServer until it is available and ready.
pub async fn init() -> Result<AxonServerHandle> {
    let axon_server_handle = wait_for_server("proxy", 8124, "API").await.unwrap();
    debug!("Axon connection: {:?}", axon_server_handle);
    Ok(axon_server_handle)
}

#[tonic::async_trait]
impl CommandSink for AxonServerHandle {
    async fn send_command(&self, command_type: &str, command: Box<&(dyn VecU8Message + Sync)>) -> Result<Option<SerializedObject>> {
        debug!("Sending command: {:?}: {:?}", command_type, self.display_name);
        let mut buf = Vec::new();
        command.encode_u8(&mut buf).unwrap();
        let buffer_length = buf.len();
        debug!("Buffer length: {:?}", buffer_length);
        let serialized_command = SerializedObject {
            r#type: command_type.to_string(),
            revision: "1".to_string(),
            data: buf,
        };
        submit_command(self, &serialized_command).await
    }
}

async fn submit_command(this: &AxonServerHandle, message: &SerializedObject) -> Result<Option<SerializedObject>> {
    debug!("Message: {:?}", message);
    let mut client = CommandServiceClient::new(this.conn.clone());
    debug!("Command Service Client: {:?}", client);
    let uuid = Uuid::new_v4();
    let command = Command {
        message_identifier: format!("{}", uuid),
        name: message.r#type.clone(),
        payload: Some(message.clone()),
        client_id: this.client_id.clone(),
        component_name: this.display_name.clone(),
        meta_data: HashMap::new(),
        processing_instructions: Vec::new(),
        timestamp: 0,
    };
    let response = client.dispatch(command).await?;
    debug!("Response: {:?}", response);
    let response = response.into_inner();
    if let Some(error_message) = response.error_message {
        return Err(anyhow!(error_message.message));
    }
    Ok(response.payload)
}
