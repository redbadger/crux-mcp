use crux_core::{Command, Request, capability::Operation, command::RequestBuilder};
use rust_mcp_sdk::macros::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
pub struct UpdateRequest {
    pub data: String,
}

#[derive(Deserialize)]
pub struct UpdateResponse(pub Result<Vec<u8>, String>);

impl Operation for UpdateRequest {
    type Output = UpdateResponse;
}

pub fn call_update<Effect, Event>(
    data: String,
) -> RequestBuilder<Effect, Event, impl Future<Output = UpdateResponse>>
where
    Effect: From<Request<UpdateRequest>> + Send + 'static,
    Event: Send + 'static,
{
    Command::request_from_shell(UpdateRequest { data })
}
