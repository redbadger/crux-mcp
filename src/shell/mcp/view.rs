use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use serde::{Deserialize, Serialize};

use crate::core::capabilities::view::ViewRequest;

#[mcp_tool(
    name = "view",
    description = "Gets the ViewModel by calling the view function in a Crux app",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct View {}

impl From<View> for ViewRequest {
    fn from(_: View) -> Self {
        Self
    }
}
