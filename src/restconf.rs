use reqwest::header::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Interfaces {
    interface: Vec<Interface>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interface {
    name: String,
    state: InterfaceState,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct InterfaceState {
    name: String,
    admin_status: String,
    counters: InterfaceCounters,
    enabled: String,
    hardware_port: String,
    ifindex: uint16,
    mtu: uint32,
    oper_status: String,
    inactive: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct InterfaceCounters {
    in_broadcast_pkts: uint32,
    in_discards: uint32,
    in_errors: uint32,
    in_fcs_errors: uint32,
    in_multicast_pkts: uint32,
    in_octets: uint32,
    in_unicast_pkts: uint32,
    out_broadcast_pkts: uint32,
    out_discards: uint32,
    out_errors: uint32,
    out_fcs_errors: uint32,
    out_multicast_pkts: uint32,
    out_octets: uint32,
    out_unicast_pkts: uint32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AristaOrigin {
    Arista,
    OpenConfig,
    Fmp,
}
struct Client {
    url: Url,
    token: String,
    accept_invalid_certs: bool,
}

fn get() {}
fn post() {}
