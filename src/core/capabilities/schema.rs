use crux_core::{Command, Request, capability::Operation, command::RequestBuilder};

use crate::Result;

pub struct SchemaRequest;

pub struct SchemaResponse(pub Result<Vec<u8>>);

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
