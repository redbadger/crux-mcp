use crux_core::{Command, Request, capability::Operation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct RenderRequest(pub Result<Vec<u8>, String>);

impl Operation for RenderRequest {
    type Output = ();
}

pub fn render<Effect, Event>(result: Result<Vec<u8>, String>) -> Command<Effect, Event>
where
    Effect: From<Request<RenderRequest>> + Send + 'static,
    Event: Send + 'static,
{
    Command::notify_shell(RenderRequest(result)).build()
}
