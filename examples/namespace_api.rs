use hcn::{api, schema::*};
use windows::core::GUID;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    //turn logging on with $env:RUST_LOG="debug"      
    let _ = env_logger::try_init();
    let namespace = HostComputeNamespace::default();

    // create a network with API
    let namespace = serde_json::to_string(&namespace).unwrap();
    println!("Creating namespace: {}", namespace);
    let namesapce_handle = api::create_namespace(&GUID::zeroed(), &namespace)?;

    // we don't get info back so need to query to get metadata about network
    let query = HostComputeQuery::default();
    let query = serde_json::to_string(&query).unwrap();

    println!("Query for network info: {}", query);
    let namespace = api::query_namespace_properties(namesapce_handle, &query)?;
    println!("Query success: {}", namespace);
    let namespace: HostComputeNamespace = serde_json::from_str(&namespace).unwrap();
    api::close_namespace(namesapce_handle)?;

    println!("Deleting network: {}", namespace.id);
    api::delete_namespace(&GUID::from(namespace.id.as_str()))?;

    Ok(())
}
