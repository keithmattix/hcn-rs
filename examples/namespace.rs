use hcn::*;
use hcn::{api, schema::*};
use windows::core::GUID;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    //turn logging on with $env:RUST_LOG="debug"      
    let _ = env_logger::try_init();
    let mut api_namespace = HostComputeNamespace::default();
    api_namespace.create_with_compartment = Some(true);

    // create a network with API
    let api_namespace = serde_json::to_string(&api_namespace).unwrap();
    let namespace_handle = api::create_namespace(&GUID::zeroed(), &api_namespace)?;

    // we don't get info back so need to query to get metadata about network
    let query = HostComputeQuery::default();
    let query = serde_json::to_string(&query).unwrap();

    let api_namespace = api::query_namespace_properties(namespace_handle, &query)?;
   
    let api_namespace: HostComputeNamespace = serde_json::from_str(&api_namespace).unwrap();

    api::close_namespace(namespace_handle)?;

    // We can use the library to get the namespace info
    // it will handle all the opening/closing of handles and querying/serialization
    let namespace = get_namespace(api_namespace.id.as_str())?;

    // Values should be the same as if we used the API as we did above
    assert_eq!(api_namespace.id, namespace.id);
    assert!(namespace.namespace_id.is_some());
    assert_eq!(api_namespace.namespace_id, namespace.namespace_id);

    println!("Deleting network: {}", namespace.id);
    api::delete_namespace(&GUID::from(namespace.id.as_str()))?;

    Ok(())
}
