use super::AxonServerHandleTraitBox;
use crate::axon_server::event::{Event, GetAggregateEventsRequest};
use anyhow::Result;

/// Fetch all events for a given aggregate.
pub async fn query_events(
    axon_server_handle: AxonServerHandleTraitBox,
    aggregate_identifier: &str,
) -> Result<Vec<Event>> {
    let request = GetAggregateEventsRequest {
        aggregate_id: aggregate_identifier.to_string(),
        allow_snapshots: false,
        initial_sequence: 0,
        max_sequence: i64::MAX,
        min_token: 0,
    };
    let mut result = Vec::new();
    let mut stream = axon_server_handle.list_aggregate_events(request).await?.into_inner();
    while let Some(event) = stream.message().await? {
        result.push(event.clone());
    }
    Ok(result)
}
