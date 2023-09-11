use hcn::{api, schema::*};
use windows::core::GUID;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    //turn logging on with $env:RUST_LOG="debug"      
    let _ = env_logger::try_init();
    let network = HostComputeNetwork {
        network_type: Some(NetworkType::NAT),
        name: "test".to_string(),
        ipams: vec![Ipam::default()],
        ..Default::default()
    };

    // create a network
    let network = serde_json::to_string(&network).unwrap();
    println!("Creating network: {}", network);
    let network_handle = api::create_network(&GUID::zeroed(), &network)?;

    // we don't get info back so need to query to get metadata about network
    let query = HostComputeQuery::default();
    let query = serde_json::to_string(&query).unwrap();

    let network = api::query_network_properties(network_handle, &query)?;
    let network: HostComputeNetwork = serde_json::from_str(&network).unwrap();
    api::close_network(network_handle)?;

    println!("Deleting network: {}", network.id);
    api::delete_network(&GUID::from(network.id.as_str()))?;

    Ok(())
}
