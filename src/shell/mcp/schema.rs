use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use serde::{Deserialize, Serialize};

use crate::core::capabilities::schema::SchemaRequest;

#[mcp_tool(
    name = "schema",
    description = "Gets the schema of a Crux app",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Schema {}

impl From<Schema> for SchemaRequest {
    fn from(_: Schema) -> Self {
        Self
    }
}
