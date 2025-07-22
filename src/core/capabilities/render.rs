use crux_core::{Command, Request, capability::Operation};

use crate::Result;

pub struct RenderRequest(pub Result<Vec<u8>>);

impl Operation for RenderRequest {
    type Output = ();
}

pub fn render<Effect, Event>(result: Result<Vec<u8>>) -> Command<Effect, Event>
where
    Effect: From<Request<RenderRequest>> + Send + 'static,
    Event: Send + 'static,
{
    Command::notify_shell(RenderRequest(result)).build()
}
