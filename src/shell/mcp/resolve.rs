use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use serde::{Deserialize, Serialize};

use crate::core::capabilities::resolve::ResolveRequest;

#[mcp_tool(
    name = "resolve",
    description = "Calls the resolve function in a Crux app",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Resolve {
    pub effect_id: u32,
    pub data: String,
}

impl From<Resolve> for ResolveRequest {
    fn from(resolve: Resolve) -> Self {
        ResolveRequest {
            effect_id: resolve.effect_id,
            data: resolve.data.into_bytes(),
        }
    }
}
