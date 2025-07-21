use crux_core::{Command, Request, capability::Operation, command::RequestBuilder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct SchemaRequest;

#[derive(Deserialize)]
pub struct SchemaResponse(pub Result<String, String>);

impl Operation for SchemaRequest {
    type Output = SchemaResponse;
}

pub fn get_schema<Effect, Event>()
-> RequestBuilder<Effect, Event, impl Future<Output = SchemaResponse>>
where
    Effect: From<Request<SchemaRequest>> + Send + 'static,
    Event: Send + 'static,
{
    Command::request_from_shell(SchemaRequest)
}
