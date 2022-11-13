use super::{wait_for_server, AxonServerHandle, AxonServerHandleTrait, CommandSink, VecU8Message};
use crate::axon_server::command::Command;
use crate::axon_server::common::{meta_data_value, MetaDataValue};
use crate::axon_server::SerializedObject;
use crate::intellij_work_around::Debuggable;
use anyhow::{anyhow, Result};
use log::debug;
use std::collections::HashMap;
use std::vec::Vec;
use uuid::Uuid;

/// Polls AxonServer until it is available and ready.
pub async fn init() -> Result<AxonServerHandle> {
    let axon_server_handle = wait_for_server("proxy", 8124, "API").await.unwrap();
    debug!("Axon connection: {:?}", axon_server_handle);
    Ok(axon_server_handle)
}

pub struct SubmitCommand {
    command_type: String,
    command: Option<Box<dyn VecU8Message + Sync + Send>>,
    metadata: HashMap<String, MetaDataValue>,
}

impl Default for SubmitCommand {
    fn default() -> Self {
        let command_type = "".to_string();
        let metadata = HashMap::new();
        SubmitCommand {
            command_type,
            command: None,
            metadata,
        }
    }
}

impl SubmitCommand {
    pub fn new(command_type: &str, command: Box<dyn VecU8Message + Sync + Send>) -> SubmitCommand {
        let metadata = HashMap::new();
        SubmitCommand {
            command_type: command_type.to_string(),
            command: Some(command),
            metadata,
        }
    }
    pub fn command<'a>(
        &'a mut self,
        command_type: &str,
        command: Box<dyn VecU8Message + Sync + Send>,
    ) -> &'a mut Self {
        self.command_type = command_type.to_string();
        self.command = Some(command);
        self
    }
    pub fn annotation<'a>(&'a mut self, key: &str, value: &MetaDataValue) -> &'a mut SubmitCommand {
        let meta_data = &mut self.metadata;
        meta_data.insert(key.to_string(), value.clone());
        self
    }
    pub fn text_annotation<'a>(&'a mut self, key: &str, value: &str) -> &'a mut SubmitCommand {
        self.metadata.insert(
            key.to_string(),
            MetaDataValue {
                data: Some(meta_data_value::Data::TextValue(value.to_string())),
            },
        );
        self
    }
    pub fn correlation_id(&mut self, correlation_id: &str) -> &mut SubmitCommand {
        self.text_annotation("dendrite::correlation_id", correlation_id);
        self
    }
    pub async fn send(
        &self,
        axon_server_handle: &AxonServerHandle,
    ) -> Result<Option<SerializedObject>> {
        let command_type = self.command_type.clone();
        if command_type == "" {
            return Err(anyhow!("Empty command type"));
        }
        let meta_data = self.metadata.clone();
        if let Some(command) = &self.command {
            debug!(
                "Sending command: {:?}: {:?}",
                &self.command_type, axon_server_handle.display_name
            );
            let mut buf = Vec::new();
            command.encode_u8(&mut buf).unwrap();
            let buffer_length = buf.len();
            debug!("Buffer length: {:?}", buffer_length);
            let serialized_command = SerializedObject {
                r#type: command_type,
                revision: "1".to_string(),
                data: buf,
            };
            submit_command(axon_server_handle, serialized_command, meta_data).await
        } else {
            Err(anyhow!("Missing command"))
        }
    }
}

#[tonic::async_trait]
impl CommandSink for AxonServerHandle {
    async fn send_command(
        &self,
        command_type: &str,
        command: &(dyn VecU8Message + Sync),
    ) -> Result<Option<SerializedObject>> {
        debug!(
            "Sending command: {:?}: {:?}",
            command_type, self.display_name
        );
        let mut buf = Vec::new();
        command.encode_u8(&mut buf).unwrap();
        let buffer_length = buf.len();
        debug!("Buffer length: {:?}", buffer_length);
        let serialized_command = SerializedObject {
            r#type: command_type.to_string(),
            revision: "1".to_string(),
            data: buf,
        };
        submit_command(self, serialized_command, HashMap::new()).await
    }
}

async fn submit_command(
    this: &dyn AxonServerHandleTrait,
    message: SerializedObject,
    meta_data: HashMap<String, MetaDataValue>,
) -> Result<Option<SerializedObject>> {
    debug!("Message: {:?}", Debuggable::from(&message));
    let uuid = Uuid::new_v4();
    let command = Command {
        message_identifier: uuid.to_string(),
        name: message.r#type.clone(),
        payload: Some(message),
        client_id: this.client_id().to_string(),
        component_name: this.display_name().to_string(),
        meta_data,
        processing_instructions: Vec::new(),
        timestamp: 0,
    };
    let response = this.dispatch(command).await?.into_inner();
    debug!("Response: {:?}", Debuggable::from(&response));
    if let Some(error_message) = response.error_message {
        return Err(anyhow!(error_message.message));
    } else {
        Ok(response.payload)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;
    use super::super::AxonServerHandle;
    use crate::axon_server::command::{CommandProviderInbound, CommandProviderOutbound, CommandResponse};
    use crate::axon_server::command::command_service_server::{CommandService, CommandServiceServer};
    use futures_core::stream::Stream;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex};
    use tokio::sync::mpsc;
    use tonic::{
        transport::{Endpoint, Server, Uri},
        Request, Response, Status, Streaming
    };
    use tower::service_fn;
    use crate::axon_utils::WorkerRegistry;

    #[tokio::test]
    async fn test_submit_command() -> Result<()> {
        let (client, server) = tokio::io::duplex(1024);

        let api_server = MockCommandServer::default();

        tokio::spawn(async move {
            Server::builder()
                .add_service(CommandServiceServer::new(api_server))
                .serve_with_incoming(futures::stream::iter(vec![Ok::<_, std::io::Error>(server)]))
                .await
        });

        #[derive(Default)]
        struct MockCommandServer {}

        #[tonic::async_trait]
        impl CommandService for MockCommandServer {
            type OpenStreamStream =
            Pin<Box<dyn Stream<Item = Result<CommandProviderInbound, Status>> + Send + Sync + 'static>>;

            async fn open_stream(&self, _request: Request<Streaming<CommandProviderOutbound>>) -> std::result::Result<Response<Self::OpenStreamStream>, Status> {
                todo!()
            }

            async fn dispatch(&self, request: Request<Command>) -> std::result::Result<Response<CommandResponse>, Status> {
                let message_identifier = request.into_inner().message_identifier;
                let mut ack = CommandResponse::default();
                ack.message_identifier = message_identifier;
                let mut payload = SerializedObject::default();
                payload.r#type = "test-payload".to_string();
                ack.payload = Some(payload);
                Ok(Response::new(ack))
            }
        }

        // Move client to an option so we can _move_ the inner value
        // on the first attempt to connect. All other attempts will fail.
        let mut client = Some(client);
        let channel = Endpoint::try_from("http://[::]:50051")?
            .connect_with_connector(service_fn(move |_: Uri| {
                let client = client.take();

                async move {
                    if let Some(client) = client {
                        Ok(client)
                    } else {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Client already taken",
                        ))
                    }
                }
            }))
            .await?;

        let (tx, rx) = mpsc::channel(10);
        let registry = WorkerRegistry {
            workers: HashMap::new(),
            notifications: rx
        };

        let axon_server_handle = AxonServerHandle {
            display_name: "Test AxonServer handle".to_string(),
            client_id: "test-client".to_string(),
            conn: channel,
            notify: tx,
            registry: Arc::new(Mutex::new(registry))
        };

        let payload_type = "test-payload";
        let payload = SerializedObject {
            r#type: payload_type.to_string(),
            revision: "".to_string(),
            data: vec![],
        };
        let mut command_response = CommandResponse::default();
        command_response.payload = Some(payload);
        // let tonic_command_response = tonic::Response::new(command_response);

        let message = SerializedObject {
            r#type: "unknown".to_string(),
            revision: "1".to_string(),
            data: vec![],
        };
        let meta_data = HashMap::default();

        let actual_payload = submit_command(&axon_server_handle, message, meta_data).await?;
        let serialized_object = actual_payload.expect("Missing serialized object");
        assert_eq!(&serialized_object.r#type, payload_type);
        Ok(())
    }
}
