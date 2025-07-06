use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, schema_utils::CallToolError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::Core;

#[derive(Error, Debug)]
#[error(transparent)]
struct Error(#[from] anyhow::Error);

#[mcp_tool(
    name = "core",
    description = "Exposes a Crux app as a core, with a single view function",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CruxCoreTool {}

impl CruxCoreTool {
    #[allow(clippy::unnecessary_wraps, clippy::unused_self)]
    pub fn call_tool(&self, context: &Core) -> Result<CallToolResult, CallToolError> {
        let view = context.view().map_err(|e| CallToolError::new(Error(e)))?;
        Ok(CallToolResult::text_content(vec![view.into()]))
    }
}
