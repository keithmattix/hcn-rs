pub mod api;
pub mod schema;
mod cotask;

use windows::core::GUID;
use anyhow::Result;
use crate::schema::*;

pub fn get_namespace(id: &str) -> Result<HostComputeNamespace> {
    let guid = GUID::from(id);
    
    let namespace_handle = api::open_namespace(&guid)?;

    let query = HostComputeQuery::default();
    let query = serde_json::to_string(&query).unwrap();
    
    let name_space = api::query_namespace_properties(namespace_handle, &query)?;
    let name_space: HostComputeNamespace = serde_json::from_str(&name_space).unwrap();
    api::close_namespace(namespace_handle)?;

    Ok(name_space)
}
