use std::{
    io,
    net::Ipv4Addr,
    ops::{BitAnd, BitOr},
};

use ipnet::IpAdd;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const IPV4_BITS: u32 = 32;
const MAX_OCTET_VALUE: u8 = 255;

#[derive(Debug, Error)]
pub enum SubnetError {
    #[error("Invalid IP address: {0}")]
    InvalidIpAddress(String),
    #[error("Invalid CIDR: {0}")]
    InvalidCidr(u32),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Subnet {
    pub network: Ipv4Addr,
    pub mask: Ipv4Addr,
    pub class: char,
    pub cidr: u32,
    pub first_host: Ipv4Addr,
    pub last_host: Ipv4Addr,
    pub broadcast: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub hosts: u32,
    pub real_hosts: u32,
    pub next_subnet: Ipv4Addr,
    pub next_cidr: u32,
}

impl Subnet {
    pub fn new(network: &str, cidr: u32, hosts: u32) -> Result<Subnet, SubnetError> {
        let network = Subnet::string_to_ip(network)?;
        let mask = Subnet::cidr_to_mask(cidr)?;

        Ok(Subnet {
            network,
            mask,
            cidr,
            broadcast: Ipv4Addr::new(0, 0, 0, 0),
            gateway: Ipv4Addr::new(0, 0, 0, 0),
            first_host: Ipv4Addr::new(0, 0, 0, 0),
            last_host: Ipv4Addr::new(0, 0, 0, 0),
            hosts,
            real_hosts: 0,
            class: Subnet::determine_class(cidr),
            next_subnet: Ipv4Addr::new(0, 0, 0, 0),
            next_cidr: 0,
        })
    }

    pub fn calculate(&mut self) -> Result<(), SubnetError> {
        let cidr_offset = (self.hosts.next_power_of_two() as f32).log2().ceil() as u32;

        let real_hosts = u32::pow(2, cidr_offset) - 2;
        self.real_hosts = real_hosts;

        let new_cidr = IPV4_BITS - cidr_offset;
        let new_mask = Subnet::cidr_to_mask(new_cidr)?;

        self.broadcast = self.network.bitor(!new_mask);

        self.gateway = self.broadcast.bitand(Ipv4Addr::new(
            MAX_OCTET_VALUE,
            MAX_OCTET_VALUE,
            MAX_OCTET_VALUE,
            254,
        ));
        self.first_host = self.network.bitor(Ipv4Addr::new(0, 0, 0, 1));
        self.last_host = self.broadcast.bitand(Ipv4Addr::new(
            MAX_OCTET_VALUE,
            MAX_OCTET_VALUE,
            MAX_OCTET_VALUE,
            253,
        ));

        self.next_subnet = self.broadcast.saturating_add(1);
        self.next_cidr = new_cidr;

        Ok(())
    }

    fn string_to_ip(ip: &str) -> Result<Ipv4Addr, SubnetError> {
        ip.parse()
            .map_err(|_| SubnetError::InvalidIpAddress(ip.to_string()))
    }

    fn cidr_to_mask(cidr: u32) -> Result<Ipv4Addr, SubnetError> {
        if cidr > IPV4_BITS {
            return Err(SubnetError::InvalidCidr(cidr));
        }

        let mask = u32::MAX.checked_shl(IPV4_BITS - cidr).unwrap_or(0);
        Ok(Ipv4Addr::from(mask))
    }

    fn determine_class(cidr: u32) -> char {
        match cidr {
            0..=8 => 'A',
            9..=16 => 'B',
            17..=24 => 'C',
            25..=32 => 'D',
            _ => 'E',
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "\n## Subnet Info:
            \n\t - Network: {}
            \n\t - Mask: {}
            \n\t - CIDR: {}
            \n\t - Class: {}
            \n\t - Broadcast: {}
            \n\t - Gateway: {}
            \n\t - First Host: {}
            \n\t - Last Host: {}
            \n\t - Hosts: {}
            \n\t - Real Hosts: {}
            \n\t - Wasted Hosts: {}",
            self.network,
            self.mask,
            self.cidr,
            self.class,
            self.broadcast,
            self.gateway,
            self.first_host,
            self.last_host,
            self.hosts,
            self.real_hosts,
            self.real_hosts + 2 - self.hosts
        )
    }

    pub fn to_markdown_table(&self) -> String {
        format!(
            "| **Network** | **Mask** | **CIDR** | **Class** | **Broadcast** | **Gateway** | **First Host** | **Last Host** | **Hosts** | **Real Hosts** | **Wasted Hosts** |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            self.network,
            self.mask,
            self.cidr,
            self.class,
            self.broadcast,
            self.gateway,
            self.first_host,
            self.last_host,
            self.hosts,
            self.real_hosts,
            self.real_hosts + 2 - self.hosts
        )
    }
}
