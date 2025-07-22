use rust_mcp_sdk::{
    McpServer, StdioTransport, TransportOptions,
    error::SdkResult,
    mcp_server::server_runtime,
    schema::{
        Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
        ServerCapabilitiesTools,
    },
};

use crate::{
    event_loop::Core,
    shell::{host::Host, mcp::handler::MyServerHandler},
};

mod core;
mod error;
mod event_loop;
mod shell;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> SdkResult<()> {
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Crux app MCP Server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            title: None,
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some("server instructions...".to_string()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let mut host = Host::default();
    host.load().expect("failed to load guest");
    let core = Core::new(host);

    let server = server_runtime::create_server(
        server_details,
        StdioTransport::new(TransportOptions::default())?,
        MyServerHandler::new(core),
    );

    server.start().await
}
