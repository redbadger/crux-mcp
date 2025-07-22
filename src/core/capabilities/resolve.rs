use crux_core::{Command, Request, capability::Operation, command::RequestBuilder};

use crate::Result;

pub struct ResolveRequest {
    pub effect_id: u32,
    pub data: Vec<u8>,
}

pub struct ResolveResponse(pub Result<Vec<u8>>);

impl Operation for ResolveRequest {
    type Output = ResolveResponse;
}

pub fn call_resolve<Effect, Event>(
    req: ResolveRequest,
) -> RequestBuilder<Effect, Event, impl Future<Output = ResolveResponse>>
where
    Effect: From<Request<ResolveRequest>> + Send + 'static,
    Event: Send + 'static,
{
    Command::request_from_shell(req)
}
