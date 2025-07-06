# MCP Server for Crux applications

> Note: not fully implemented yet

Loads a Crux application's shared library as a WebAssembly Component, exposing it with the ModelContext Protocol to LLM agents for agentic coding.

## Prerequisites

Currently hardcoded to a `wasm32-wasip2` build of the `crux-counter`.

Checkout the branch from [this pull request](https://github.com/redbadger/crux/pull/401) into a directory called "crux", alongside this repository.

```bash
pushd ../crux/examples/counter-next
cargo build --package shared --target wasm32-wasip2
popd
```

## Running the Server

To run the MCP server:

```bash
cargo build
export CRUX_COMPONENT=/path/to/crux/examples/counter-next/target/wasm32-wasip2/debug/shared.wasm
pnpx @modelcontextprotocol/inspector ./target/debug/crux-mcp
```
