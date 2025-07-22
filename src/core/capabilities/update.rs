use crux_core::{Command, Request, capability::Operation, command::RequestBuilder};

use crate::Result;

pub struct UpdateRequest {
    pub data: Vec<u8>,
}

pub struct UpdateResponse(pub Result<Vec<u8>>);

impl Operation for UpdateRequest {
    type Output = UpdateResponse;
}

pub fn call_update<Effect, Event>(
    data: Vec<u8>,
) -> RequestBuilder<Effect, Event, impl Future<Output = UpdateResponse>>
where
    Effect: From<Request<UpdateRequest>> + Send + 'static,
    Event: Send + 'static,
{
    Command::request_from_shell(UpdateRequest { data })
}
