// see https://learn.microsoft.com/en-us/virtualization/api/hcn/hns_schema

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostComputeNetwork {
    #[serde(rename = "ID", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub network_type: Option<NetworkType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<NetworkPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac_pool: Option<MacPool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns: Option<Dns>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipams: Vec<Ipam>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<NetworkFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<Health>,
    #[serde(default)]
    pub schema_version: Version,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]

pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl Default for Version {
    fn default() -> Self {
        Self { major: 2, minor: 2 }
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Health {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<ExtraParams>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ExtraParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_containers: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layered_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_guid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_vm: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Dns {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_list: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

pub type NetworkFlags = u32;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum NetworkType {
    NAT,
    Transparent,
    L2Bridge,
    L2Tunnel,
    ICS,
    Private,
    Overlay,
}
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkPolicy {
    #[serde(rename = "Type")]
    pub network_type: NetworkPolicyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Ipam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<Subnet>,
}

impl Default for Ipam {
    fn default() -> Self {
        Self {
            r#type: Some("Static".to_string()),
            subnets: vec![Subnet::default()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Subnet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_prefix: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<serde_json::Value>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<Route>,
}

impl Default for Subnet {
    fn default() -> Self {
        Self {
            ip_address_prefix: Some("10.0.0.0/16".to_string()),
            routes: vec![Route {
                next_hop: Some("10.0.0.1".to_string()),
                destination_prefix: Some("0.0.0.0/0".to_string()),
                ..Default::default()
            }],
            policies: vec![],
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Route {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_hop: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<u16>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum NetworkPolicyType {
    SourceMacAddress,
    NetAdapterName,
    VSwitchExtension,
    DrMacAddress,
    AutomaticDNS,
    InterfaceConstraint,
    ProviderAddress,
    RemoteSubnetRoute,
    VxlanPort,
    HostRoute,
    SetPolicy,
    NetworkL4Proxy,
    LayerConstraint,
    NetworkACL,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MacRange {
    pub start_mac_address: Option<String>,
    pub end_mac_address: Option<String>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MacPool {
    pub ranges: Option<Vec<MacRange>>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostComputeNamespace {
    #[serde(rename = "ID", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<NamespaceType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<NamespaceResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_with_compartment: Option<bool>,
    #[serde(default)]
    pub schema_version: Version,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum NamespaceType {
    Host,
    HostDefault,
    Guest,
    GuestDefault,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NamespaceResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<NamespaceResourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum NamespaceResourceType {
    Container,
    Endpoint,
}

#[derive(Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u32)]
pub enum HostComputeQueryFlags {
    #[default]
    None = 0,
    Detailed = 1,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostComputeQuery {
    #[serde(default)]
    pub schema_version: Version,
    #[serde(default)]
    pub flags: HostComputeQueryFlags,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub filter: String,
}
