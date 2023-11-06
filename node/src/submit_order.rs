use jsonrpsee::{
	core::{params::ArrayParams, Error as JsonRpseeError, client::{Client as JsonRpcClient, ClientT},},
	rpc_params,ws_client::WsClientBuilder,
};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use futures::{
	channel::oneshot::Sender as OneshotSender,
	future::BoxFuture,
	stream::FuturesUnordered,
	FutureExt, StreamExt,
};
use url::Url;
use cumulus_relay_chain_interface::RelayChainError;

const LOG_TARGET: &str = "reconnecting-websocket-client";

/// Format url and force addition of a port
fn url_to_string_with_port(url: Url) -> Option<String> {
	// This is already validated on CLI side, just defensive here
	if (url.scheme() != "ws" && url.scheme() != "wss") || url.host_str().is_none() {
		tracing::warn!(target: LOG_TARGET, ?url, "Non-WebSocket URL or missing host.");
		return None
	}

	// Either we have a user-supplied port or use the default for 'ws' or 'wss' here
	Some(format!(
		"{}://{}:{}{}{}",
		url.scheme(),
		url.host_str()?,
		url.port_or_known_default()?,
		url.path(),
		url.query().map(|query| format!("?{}", query)).unwrap_or_default()
	))
}
#[allow(dead_code)]
struct ClientManager {
	urls: Vec<String>,
	active_client: Arc<JsonRpcClient>,
	active_index: usize,
}
async fn connect_next_available_rpc_server(
	urls: &Vec<String>,
	starting_position: usize,
) -> Result<(usize, Arc<JsonRpcClient>), ()> {
	tracing::debug!(target: LOG_TARGET, starting_position, "Connecting to RPC server.");
	for (counter, url) in urls.iter().cycle().skip(starting_position).take(urls.len()).enumerate() {
		let index = (starting_position + counter) % urls.len();
		tracing::debug!(
			target: LOG_TARGET,
			index,
			url,
			"Trying to connect to next external relaychain node.",
		);
		match WsClientBuilder::default().build(&url).await {
			Ok(ws_client) => return Ok((index, Arc::new(ws_client))),
			Err(err) => tracing::debug!(target: LOG_TARGET, url, ?err, "Unable to connect."),
		};
	}
	Err(())
}

impl ClientManager {
	pub async fn new(urls: Vec<String>) -> Result<Self, ()> {
		if urls.is_empty() {
			return Err(())
		}
		let active_client = connect_next_available_rpc_server(&urls, 0).await?;
		Ok(Self { urls, active_client: active_client.1, active_index: active_client.0 })
	}

	fn create_request(
		&self,
		method: String,
		params: ArrayParams,
		response_sender: OneshotSender<Result<JsonValue, JsonRpseeError>>,
	) -> BoxFuture<'static, Result<(), ()>> {
		let future_client = self.active_client.clone();
		async move {
			let resp = future_client.request(&method, params.clone()).await;
			if let Err(JsonRpseeError::RestartNeeded(_)) = resp {
				return Err(())
			}

			if let Err(err) = response_sender.send(resp) {
				tracing::debug!(
					target: LOG_TARGET,
					?err,
					"Recipient no longer interested in request result"
				);
			}
			Ok(())
		}
		.boxed()
	}
}

pub async fn submit_extrinsic_rpc_call(
	url:&str,
	method: String,
	params: ArrayParams,
	response_sender: OneshotSender<Result<JsonValue, JsonRpseeError>>,
) {
    let urls = vec![Url::parse(url).unwrap()];
	let urls_col = urls.into_iter().filter_map(url_to_string_with_port).collect();
	let mut pending_requests = FuturesUnordered::new();
	let Ok(client_manager) = ClientManager::new(urls_col).await else {
		tracing::error!(target: LOG_TARGET, "No valid RPC url found. Stopping RPC worker.");
		return
	};
	pending_requests.push(client_manager.create_request(method, params, response_sender));
	while !pending_requests.is_empty() {
		pending_requests.next().await ;
	}
}
pub async fn build_rpc_for_submit_order(
	url:&str,
	extrinsic:String
) -> Result<(), RelayChainError>
{
	let (tx, rx) = futures::channel::oneshot::channel();
	let params = rpc_params![extrinsic];
	submit_extrinsic_rpc_call(url, "author_submitExtrinsic".into(), params, tx).await;
	let _value = rx.await.map_err(|err| {
		RelayChainError::WorkerCommunicationError(format!(
			"Unexpected channel close on RPC worker side: {}",
			err
		))
	})??;
	Ok(())
}