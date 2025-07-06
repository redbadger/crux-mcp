use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, schema_utils::CallToolError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::Core;

#[derive(Error, Debug)]
#[error(transparent)]
struct Error(#[from] anyhow::Error);

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

impl Schema {
    pub fn call_tool(context: &Core) -> Result<CallToolResult, CallToolError> {
        let schema = context.schema().map_err(|e| CallToolError::new(Error(e)))?;
        Ok(CallToolResult::text_content(vec![schema.into()]))
    }
}

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
    data: String,
}

impl Update {
    pub fn call_tool(self, context: &Core) -> Result<CallToolResult, CallToolError> {
        let update = context
            .update(self.data.into_bytes())
            .map_err(|e| CallToolError::new(Error(e)))?;
        let update = String::from_utf8(update).map_err(|e| CallToolError::new(Error(e.into())))?;
        Ok(CallToolResult::text_content(vec![update.into()]))
    }
}

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
    effect_id: u32,
    data: String,
}

impl Resolve {
    pub fn call_tool(self, context: &Core) -> Result<CallToolResult, CallToolError> {
        let resolve = context
            .resolve(self.effect_id, self.data.into_bytes())
            .map_err(|e| CallToolError::new(Error(e)))?;
        let resolve =
            String::from_utf8(resolve).map_err(|e| CallToolError::new(Error(e.into())))?;
        Ok(CallToolResult::text_content(vec![resolve.into()]))
    }
}

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

impl View {
    pub fn call_tool(context: &Core) -> Result<CallToolResult, CallToolError> {
        let view = context.view().map_err(|e| CallToolError::new(Error(e)))?;
        let view = String::from_utf8(view).map_err(|e| CallToolError::new(Error(e.into())))?;
        Ok(CallToolResult::text_content(vec![view.into()]))
    }
}
