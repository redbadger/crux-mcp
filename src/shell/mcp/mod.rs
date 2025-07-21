pub mod handler;
pub mod resolve;
pub mod schema;
pub mod update;
pub mod view;

use resolve::Resolve;
use schema::Schema;
use update::Update;
use view::View;

use crate::core::app::Event;

rust_mcp_sdk::tool_box!(MyTools, [Schema, Update, Resolve, View]);

impl From<MyTools> for Event {
    fn from(tool: MyTools) -> Self {
        match tool {
            MyTools::Schema(params) => Event::Schema(params.into()),
            MyTools::Update(params) => Event::Update(params.into()),
            MyTools::Resolve(params) => Event::Resolve(params.into()),
            MyTools::View(params) => Event::View(params.into()),
        }
    }
}
