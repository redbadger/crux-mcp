pub mod handler;
pub mod resolve;
pub mod schema;
pub mod update;
pub mod view;

use resolve::Resolve;
use schema::Schema;
use update::Update;
use view::View;

rust_mcp_sdk::tool_box!(MyTools, [Schema, Update, Resolve, View]);
