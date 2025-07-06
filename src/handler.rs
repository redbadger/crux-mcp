use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};

use crate::core::Core;
use crate::tools::core::Schema;
use crate::tools::{MyTools, core::View};

// Custom Handler to handle MCP Messages
pub struct MyServerHandler {
    core: Core,
}

impl MyServerHandler {
    pub fn new() -> Self {
        let mut core = Core::default();
        core.load().unwrap();
        Self { core }
    }
}

// To check out a list of all the methods in the trait that you can override, take a look at
// https://github.com/rust-mcp-stack/rust-mcp-sdk/blob/main/crates/rust-mcp-sdk/src/mcp_handlers/mcp_server_handler.rs

#[async_trait]
#[allow(unused)]
impl ServerHandler for MyServerHandler {
    // Handle ListToolsRequest, return list of available tools as ListToolsResult
    async fn handle_list_tools_request(
        &self,
        request: ListToolsRequest,
        runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: MyTools::tools(),
        })
    }

    /// Handles incoming CallToolRequest and processes it using the appropriate tool.
    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let params = MyTools::try_from(request.params).map_err(CallToolError::new)?;
        let core = &self.core;

        match params {
            MyTools::Schema(_) => Schema::call_tool(core),
            MyTools::Update(params) => params.call_tool(core),
            MyTools::Resolve(params) => params.call_tool(core),
            MyTools::View(_) => View::call_tool(core),
        }
    }
}
