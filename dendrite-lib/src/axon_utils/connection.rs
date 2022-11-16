use super::AxonServerHandle;
use crate::axon_server::control::platform_inbound_instruction;
use crate::axon_server::control::platform_service_client::PlatformServiceClient;
use crate::axon_server::control::{ClientIdentification, PlatformInboundInstruction};
use crate::intellij_work_around::Debuggable;
use anyhow::{anyhow, Result};
use async_channel::bounded;
use async_stream::stream;
use futures_core::stream::Stream;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time;
use tokio::select;
use tokio::time::sleep;
use tonic::transport::Channel;
use tonic::{Request, Response};
use uuid::Uuid;
use crate::axon_utils::{WorkerControl, WorkerRegistry};

/// Polls AxonServer until it is available and ready.
pub async fn wait_for_server(host: &str, port: u32, label: &str) -> Result<AxonServerHandle> {
    let url = format!("http://{}:{}", host, port);
    let client_id = format!("{}", Uuid::new_v4());
    let conn = wait_for_connection(&url, label, &client_id).await;
    debug!(
        "Axon server handle: {:?}: {:?}: {:?}",
        label, client_id, conn
    );
    let (tx, rx) = bounded(10);
    let registry = WorkerRegistry {
        workers: HashMap::new(),
        notifications: rx
    };
    let registry = Arc::new(Mutex::new(registry));
    let connection = AxonServerHandle {
        display_name: label.to_string(),
        client_id,
        conn,
        notify: tx,
        registry
    };
    Ok(connection)
}

async fn wait_for_connection(url: &str, label: &str, client_id: &str) -> Channel {
    let interval = time::Duration::from_secs(1);
    loop {
        if let Some(conn) = try_to_connect(url, label, client_id).await {
            return conn;
        }
        sleep(interval).await;
        continue;
    }
}

async fn try_to_connect(url: &str, label: &str, client_id: &str) -> Option<Channel> {
    connect(url, label, client_id)
        .await
        .map_err(|e| {
            debug!("Error while trying to connect to AxonServer: {:?}", e);
        })
        .ok()
        .flatten()
}

async fn connect(url: &str, label: &str, client_id: &str) -> Result<Option<Channel>> {
    let conn = tonic::transport::Endpoint::from_shared(url.to_string())?
        .connect()
        .await
        .map_err(|_| debug!(". Can't connect to AxonServer (yet)"))
        .ok();
    let conn = match conn {
        Some(conn) => conn,
        None => return Ok(None),
    };
    let mut client = PlatformServiceClient::new(conn.clone());
    let client_identification = ClientIdentification {
        component_name: format!("Rust client {}", label),
        client_id: client_id.to_string(),
        ..Default::default()
    };
    let response = client
        .get_platform_server(Request::new(client_identification))
        .await
        .map_err(|_| debug!(". AxonServer is not available (yet)"))
        .ok();
    if response.is_none() {
        return Ok(None);
    }
    let platform_info = response.map(Response::into_inner);
    let platform_info = platform_info.as_ref();
    debug!("Response: {:?}", platform_info.map(|p| Debuggable::from(p)));
    return Ok(Some(conn));
}

pub fn platform_worker_for(
    label: &str
) -> Box<dyn FnOnce(AxonServerHandle,WorkerControl) -> Pin<Box<dyn Future<Output = ()> + Send>> + Sync> {
    let label = label.to_string();
    Box::new(move |handle, worker_control: WorkerControl| Box::pin(mute_platform_worker(handle.clone(), label.clone(), worker_control)))
}

pub async fn mute_platform_worker(axon_server_handle: AxonServerHandle, label: String, worker_control: WorkerControl) {
    if let Err(e) = platform_worker(axon_server_handle, &label, worker_control).await {
        warn!("Platform worker: Error: {:?}", e);
    }
}

/// Subscribes  to commands, verifies them against the command projection and sends emitted events to AxonServer.
pub async fn platform_worker(axon_server_handle: AxonServerHandle, label: &str, worker_control: WorkerControl) -> Result<()> {
    debug!("Platform worker: start");
    let conn = axon_server_handle.conn;
    let client_id = axon_server_handle.client_id;

    let mut client = PlatformServiceClient::new(conn.clone());
    let output = create_output_stream(label.to_string(), &client_id);
    let response = client.open_stream(Request::new(output)).await?;
    debug!("Stream response: {:?}", response);

    let mut inbound = response.into_inner();
    loop {
        let control_channel = worker_control.get_control_channel().clone();
        let message = select! {
            message_result = inbound.message() => message_result,
            _command = control_channel.recv() => {
                info!("Platform worker stopped: {:?}", label);
                return Ok(())
            }
        };
        match message {
            Ok(Some(message)) => {
                debug!(
                    "Incoming (= 'outbound') platform instruction: {:?}",
                    Debuggable::from(&message)
                );
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
    label: String,
    client_id: &str,
) -> impl Stream<Item = PlatformInboundInstruction> {
    let interval = time::Duration::from_secs(300);
    let client_id = client_id.to_string().clone();
    stream! {
        let client_identification = ClientIdentification {
            component_name: format!("Rust client {}", &label),
            client_id: client_id,
            ..Default::default()
        };
        let instruction_id = Uuid::new_v4();
        let instruction = PlatformInboundInstruction {
            instruction_id: format!("{}", instruction_id),
            request: Some(platform_inbound_instruction::Request::Register(client_identification)),
        };
        yield instruction.to_owned();

        loop {
            sleep(interval).await;
            debug!(".");
        }
    }
}
