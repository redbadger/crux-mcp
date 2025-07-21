use crux_core::{Command, Request, capability::Operation, command::RequestBuilder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResolveRequest {
    pub effect_id: u32,
    pub data: String,
}

#[derive(Deserialize)]
pub struct ResolveResponse(pub Result<Vec<u8>, String>);

impl Operation for ResolveRequest {
    type Output = ResolveResponse;
}

pub fn call_resolve<Effect, Event>(
    effect_id: u32,
    data: String,
) -> RequestBuilder<Effect, Event, impl Future<Output = ResolveResponse>>
where
    Effect: From<Request<ResolveRequest>> + Send + 'static,
    Event: Send + 'static,
{
    Command::request_from_shell(ResolveRequest { effect_id, data })
}
