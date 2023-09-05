use crate::cotask::{AsOption, CoTaskMemWString};
use anyhow::{Context, Ok, Result};
use std::ffi::c_void;
use windows::core::{GUID, HSTRING};
/// Modified from https://github.com/rafawo/hcs-rs under MIT license
use windows::Win32::System::HostComputeNetwork::{
    HcnCloseEndpoint, HcnCloseLoadBalancer, HcnCloseNamespace, HcnCloseNetwork, HcnCreateEndpoint,
    HcnCreateLoadBalancer, HcnCreateNamespace, HcnCreateNetwork, HcnDeleteEndpoint,
    HcnDeleteLoadBalancer, HcnDeleteNamespace, HcnDeleteNetwork, HcnEnumerateEndpoints,
    HcnEnumerateLoadBalancers, HcnEnumerateNamespaces, HcnEnumerateNetworks, HcnModifyEndpoint,
    HcnModifyLoadBalancer, HcnModifyNamespace, HcnModifyNetwork, HcnOpenEndpoint,
    HcnOpenLoadBalancer, HcnOpenNamespace, HcnOpenNetwork, HcnQueryEndpointProperties,
    HcnQueryLoadBalancerProperties, HcnQueryNamespaceProperties, HcnQueryNetworkProperties,
    HcnRegisterServiceCallback, HcnUnregisterServiceCallback, HCN_NOTIFICATION_CALLBACK,
};

/// Handle to a callback registered on an hns object
pub struct HcnCallback(*const c_void);

/// Function type for HNS notification callbacks
pub type HcnNotificationCallback = HCN_NOTIFICATION_CALLBACK;

/// Context handle referencing a Network in HNS
#[derive(Clone, Copy)]
pub struct HcnNetworkHandle(*const c_void);

/// Context handle referencing a Namespace in HNS
#[derive(Clone, Copy)]
pub struct HcnNamespaceHandle(*const c_void);

/// Context handle referencing an Endpoint in HNS
#[derive(Clone, Copy)]
pub struct HcnEndpointHandle(*const c_void);

/// Context handle referencing a LoadBalancer in HNS
#[derive(Clone, Copy)]
pub struct HcnLoadBalancerHandle(*const c_void);

/// Context handle referencing the HNS service
#[derive(Clone, Copy)]
pub struct HcnServiceHandle(*const c_void);

/// Return a list of existing Networks.
pub fn enumerate_networks(query: &str) -> Result<String> {
    unsafe {
        let mut networks = CoTaskMemWString::new();
        let mut error_record = CoTaskMemWString::new();

        HcnEnumerateNetworks(
            &HSTRING::from(query),
            networks.as_ptr(),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(networks.to_string())
    }
}

/// Create a network.
pub fn create_network(id: &GUID, settings: &str) -> Result<HcnNetworkHandle> {
    unsafe {
        let mut network_handle = std::ptr::null_mut();
        let mut error_record = CoTaskMemWString::new();

        HcnCreateNetwork(
            id,
            &HSTRING::from(settings),
            &mut network_handle,
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(HcnNetworkHandle(network_handle))
    }
}

/// Lookup an existing network.
pub fn open_network(id: &GUID) -> Result<HcnNetworkHandle> {
    unsafe {
        let network_handle = std::ptr::null_mut();
        let mut error_record = CoTaskMemWString::new();

        HcnOpenNetwork(
            id,
            &mut (network_handle as *mut c_void),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(HcnNetworkHandle(network_handle))
    }
}

/// Modify the settings of a Network.
pub fn modify_network(network: HcnNetworkHandle, settings: &str) -> Result<()> {
    unsafe {
        let mut error_record = CoTaskMemWString::new();

        HcnModifyNetwork(
            network.0,
            &HSTRING::from(settings),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(())
    }
}

/// Query network settings.
pub fn query_network_properties(network: HcnNetworkHandle, query: &str) -> Result<String> {
    unsafe {
        let mut properties = CoTaskMemWString::new();
        let mut error_record = CoTaskMemWString::new();

        HcnQueryNetworkProperties(
            network.0,
            &HSTRING::from(query),
            properties.as_ptr(),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(properties.to_string())
    }
}

/// Delete a network.
pub fn delete_network(id: &GUID) -> Result<()> {
    unsafe {
        let mut error_record = CoTaskMemWString::new();

        HcnDeleteNetwork(id, error_record.as_option()).context(error_record.to_string())?;

        Ok(())
    }
}

/// Close handle to a Network.
pub fn close_network(network: HcnNetworkHandle) -> Result<()> {
    unsafe {
        HcnCloseNetwork(network.0)?;

        Ok(())
    }
}

/// Return a list of existing Namespaces.
pub fn enumerate_namespaces(query: &str) -> Result<String> {
    unsafe {
        let mut namespaces = CoTaskMemWString::new();
        let mut error_record = CoTaskMemWString::new();

        HcnEnumerateNamespaces(
            &HSTRING::from(query),
            namespaces.as_ptr(),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(namespaces.to_string())
    }
}

/// Create a Namespace.
pub fn create_namespace(id: &GUID, settings: &str) -> Result<HcnNamespaceHandle> {
    unsafe {
        let mut namespace_handle = std::ptr::null_mut();
        let mut error_record = CoTaskMemWString::new();

        HcnCreateNamespace(
            id,
            &HSTRING::from(settings),
            &mut namespace_handle,
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(HcnNamespaceHandle(namespace_handle))
    }
}

/// Lookup an existing Namespace.
pub fn open_namespace(id: &GUID) -> Result<HcnNamespaceHandle> {
    unsafe {
        let mut namespace_handle = std::ptr::null_mut();
        let mut error_record = CoTaskMemWString::new();

        HcnOpenNamespace(id, &mut namespace_handle, error_record.as_option())
            .context(error_record.to_string())?;

        Ok(HcnNamespaceHandle(namespace_handle))
    }
}

/// Modify the settings of a Namespace.
pub fn modify_namespace(namespace: HcnNamespaceHandle, settings: &str) -> Result<()> {
    unsafe {
        let mut error_record = CoTaskMemWString::new();

        HcnModifyNamespace(
            namespace.0,
            &HSTRING::from(settings),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(())
    }
}

/// Query Namespace settings.
pub fn query_namespace_properties(namespace: HcnNamespaceHandle, query: &str) -> Result<String> {
    unsafe {
        let mut properties = CoTaskMemWString::new();
        let mut error_record = CoTaskMemWString::new();

        HcnQueryNamespaceProperties(
            namespace.0,
            &HSTRING::from(query),
            properties.as_ptr(),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(properties.to_string())
    }
}

/// Delete a Namespace.
pub fn delete_namespace(id: &GUID) -> Result<()> {
    unsafe {
        let mut error_record = CoTaskMemWString::new();

        HcnDeleteNamespace(id, error_record.as_option()).context(error_record.to_string())?;

        Ok(())
    }
}

/// Close a handle to a Namespace.
pub fn close_namespace(namespace: HcnNamespaceHandle) -> Result<()> {
    unsafe {
        HcnCloseNamespace(namespace.0)?;

        Ok(())
    }
}

/// Return a list of existing Endpoints.
pub fn enumerate_endpoints(query: &str) -> Result<String> {
    unsafe {
        let mut endpoints = CoTaskMemWString::new();
        let mut error_record = CoTaskMemWString::new();

        HcnEnumerateEndpoints(
            &HSTRING::from(query),
            endpoints.as_ptr(),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(endpoints.to_string())
    }
}

/// Create an Endpoint.
pub fn create_endpoint(
    network: HcnNetworkHandle,
    id: &GUID,
    settings: &str,
) -> Result<HcnEndpointHandle> {
    unsafe {
        let mut endpoint_handle = std::ptr::null_mut();
        let mut error_record = CoTaskMemWString::new();

        HcnCreateEndpoint(
            network.0,
            id,
            &HSTRING::from(settings),
            &mut endpoint_handle,
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(HcnEndpointHandle(endpoint_handle))
    }
}

/// Lookup an existing Endpoint.
pub fn open_endpoint(id: &GUID) -> Result<HcnEndpointHandle> {
    unsafe {
        let mut endpoint_handle = std::ptr::null_mut();
        let mut error_record = CoTaskMemWString::new();

        HcnOpenEndpoint(id, &mut endpoint_handle, error_record.as_option())
            .context(error_record.to_string())?;

        Ok(HcnEndpointHandle(endpoint_handle))
    }
}

/// Modify the settings of an Endpoint.
pub fn modify_endpoint(endpoint: HcnEndpointHandle, settings: &str) -> Result<()> {
    unsafe {
        let mut error_record = CoTaskMemWString::new();

        HcnModifyEndpoint(
            endpoint.0,
            &HSTRING::from(settings),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(())
    }
}

/// Query Endpoint properties.
pub fn query_endpoint_properties(endpoint: HcnEndpointHandle, query: &str) -> Result<String> {
    unsafe {
        let mut properties = CoTaskMemWString::new();
        let mut error_record = CoTaskMemWString::new();

        HcnQueryEndpointProperties(
            endpoint.0,
            &HSTRING::from(query),
            properties.as_ptr(),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(properties.to_string())
    }
}

/// Delete an Endpoint.
pub fn delete_endpoint(id: &GUID) -> Result<()> {
    unsafe {
        let mut error_record = CoTaskMemWString::new();

        HcnDeleteEndpoint(id, error_record.as_option()).context(error_record.to_string())?;

        Ok(())
    }
}

/// Close a handle to an Endpoint.
pub fn close_endpoint(endpoint: HcnEndpointHandle) -> Result<()> {
    unsafe {
        HcnCloseEndpoint(endpoint.0)?;

        Ok(())
    }
}

/// Return a list of existing LoadBalancers.
pub fn enumerate_load_balancers(query: &str) -> Result<String> {
    unsafe {
        let mut load_balancers = CoTaskMemWString::new();
        let mut error_record = CoTaskMemWString::new();

        HcnEnumerateLoadBalancers(
            &HSTRING::from(query),
            load_balancers.as_ptr(),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(load_balancers.to_string())
    }
}

/// Create a LoadBalancer.
pub fn create_load_balancer(id: &GUID, settings: &str) -> Result<HcnLoadBalancerHandle> {
    unsafe {
        let mut load_balancer_handle = std::ptr::null_mut();
        let mut error_record = CoTaskMemWString::new();

        HcnCreateLoadBalancer(
            id,
            &HSTRING::from(settings),
            &mut load_balancer_handle,
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(HcnLoadBalancerHandle(load_balancer_handle))
    }
}

/// Lookup an existing LoadBalancer.
pub fn open_load_balancer(id: &GUID) -> Result<HcnLoadBalancerHandle> {
    unsafe {
        let mut load_balancer_handle = std::ptr::null_mut();
        let mut error_record = CoTaskMemWString::new();

        HcnOpenLoadBalancer(id, &mut load_balancer_handle, error_record.as_option())
            .context(error_record.to_string())?;

        Ok(HcnLoadBalancerHandle(load_balancer_handle))
    }
}

/// Modify the settings of a LoadBalancer.
pub fn modify_load_balancer(load_balancer: HcnLoadBalancerHandle, settings: &str) -> Result<()> {
    unsafe {
        let mut error_record = CoTaskMemWString::new();

        HcnModifyLoadBalancer(
            load_balancer.0,
            &HSTRING::from(settings),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(())
    }
}

/// Query LoadBalancer settings.
pub fn query_load_balancer_properties(
    load_balancer: HcnLoadBalancerHandle,
    query: &str,
) -> Result<String> {
    unsafe {
        let mut properties = CoTaskMemWString::new();
        let mut error_record = CoTaskMemWString::new();

        HcnQueryLoadBalancerProperties(
            load_balancer.0,
            &HSTRING::from(query),
            properties.as_ptr(),
            error_record.as_option(),
        )
        .context(error_record.to_string())?;

        Ok(properties.to_string())
    }
}

/// Delete a LoadBalancer.
pub fn delete_load_balancer(id: &GUID) -> Result<()> {
    unsafe {
        let mut error_record = CoTaskMemWString::new();

        HcnDeleteLoadBalancer(id, error_record.as_option()).context(error_record.to_string())?;

        Ok(())
    }
}

/// Close a handle to a LoadBalancer.
pub fn close_load_balancer(load_balancer: HcnLoadBalancerHandle) -> Result<()> {
    unsafe {
        HcnCloseLoadBalancer(load_balancer.0)?;

        Ok(())
    }
}

/// Registers a callback function to receive notifications of service-wide events.
pub fn register_service_callback(
    callback: HcnNotificationCallback,
    context: *const c_void,
    callback_handle: *mut HcnCallback,
) -> Result<()> {
    unsafe {
        HcnRegisterServiceCallback(callback, context, callback_handle as *mut *mut c_void)?;

        Ok(())
    }
}

/// Unregisters from service-wide notifications.
pub fn unregister_service_callback(callback_handle: HcnCallback) -> Result<()> {
    unsafe {
        HcnUnregisterServiceCallback(callback_handle.0)?;

        Ok(())
    }
}
