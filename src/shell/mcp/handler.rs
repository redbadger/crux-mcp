use std::result::Result;

use anyhow::anyhow;
use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};
use tokio::sync::mpsc;

use crate::core::{self, Core, app::Event};
use crate::error::Error;
use crate::shell::{host::Host, mcp::MyTools};

// Custom Handler to handle MCP Messages
pub struct MyServerHandler {
    core: Core,
    host: Host,
}

impl MyServerHandler {
    pub fn new(core: Core, host: Host) -> Self {
        Self { core, host }
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
    ) -> Result<ListToolsResult, RpcError> {
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
    ) -> Result<CallToolResult, CallToolError> {
        let params = MyTools::try_from(request.params).map_err(CallToolError::new)?;
        let core = &self.core;
        let host = &self.host;
        let (tx, mut rx) = mpsc::channel(1);

        match params {
            MyTools::Schema(params) => core::update(core, host, Event::Schema(params.into()), &tx),
            MyTools::Update(params) => core::update(core, host, Event::Update(params.into()), &tx),
            MyTools::Resolve(params) => {
                core::update(core, host, Event::Resolve(params.into()), &tx);
            }
            MyTools::View(params) => core::update(core, host, Event::View(params.into()), &tx),
        }

        match rx.recv().await {
            Some(result) => match result {
                Ok(data) => {
                    let data = String::from_utf8(data)
                        .map_err(|e| CallToolError::new(Error::Other(e.into())))?;
                    Ok(CallToolResult::text_content(vec![data.into()]))
                }
                Err(e) => Err(CallToolError::new(Error::Other(anyhow!("{e}")))),
            },
            None => Err(CallToolError::new(Error::Other(anyhow!("channel closed")))),
        }
    }
}
