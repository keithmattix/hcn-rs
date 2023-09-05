use hcn::{schema::*, api};
use hcn::*;
use windows::core::GUID;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let namespace = HostComputeNamespace::default();

    // create a network with API
    let namespace = serde_json::to_string(&namespace).unwrap();
    println!("Creating namespace: {}", namespace);
    let namespace_handle = api::create_namespace(&GUID::zeroed(), &namespace)?;

    // we don't get info back so need to query to get metadata about network
    let query = HostComputeQuery::default();
    let query = serde_json::to_string(&query).unwrap();
    
    println!("Query for network info: {}", query);
    let namespace = api::query_namespace_properties(namespace_handle, &query)?;
    println!("Query success: {}", namespace);
    let namespace: HostComputeNamespace = serde_json::from_str(&namespace).unwrap();
    api::close_namespace(namespace_handle)?;

    // We can use the library to get the namespace info and handle all the opening/closing of handles and querying/serialization
    let library_namespace = get_namespace(namespace.id.as_str())?;

    // Values should be the same as if we used the API as we did above
    assert_eq!(namespace.id, library_namespace.id);
    assert_eq!(namespace.namespace_type, library_namespace.namespace_type);
    assert_eq!(namespace.namespace_id, library_namespace.namespace_id);
    println!("It works: {}", library_namespace.id);

    println!("Deleting network: {}", library_namespace.id);
    api::delete_namespace(&GUID::from(library_namespace.id.as_str()))?;

    Ok(())
}