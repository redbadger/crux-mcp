use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use serde::{Deserialize, Serialize};

use crate::core::capabilities::update::UpdateRequest;

#[mcp_tool(
    name = "update",
    description = "Calls the update function in a Crux app",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Update {
    pub data: String,
}

impl From<Update> for UpdateRequest {
    fn from(update: Update) -> Self {
        Self { data: update.data }
    }
}
