use crux_core::{Command, Request, capability::Operation, command::RequestBuilder};

use crate::Result;

pub struct ViewRequest;

pub struct ViewResponse(pub Result<Vec<u8>>);

impl Operation for ViewRequest {
    type Output = ViewResponse;
}

pub fn call_view<Effect, Event>()
-> RequestBuilder<Effect, Event, impl Future<Output = ViewResponse>>
where
    Effect: From<Request<ViewRequest>> + Send + 'static,
    Event: Send + 'static,
{
    Command::request_from_shell(ViewRequest)
}
