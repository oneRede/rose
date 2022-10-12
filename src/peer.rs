use crate::config::Config;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub struct PeerInfo {
    pub name: String,
    pub ip: String,
    pub port: u32,
}

pub struct LocalPeer {
    pub inner: PeerInfo,
}

pub struct RemotePeers {
    pub inner: Vec<PeerInfo>,
    pub save_path: String,
}

impl PeerInfo {
    pub fn new(name: String, ip: String, port: u32) -> Self {
        Self { name, ip, port }
    }

    pub fn get_ip_addr(&self) -> IpAddr {
        self.ip.parse().unwrap()
    }
}

impl RemotePeers {
    fn new() -> Self {
        let config = Config::new();
        let peer_storage_path = &config.get_peer_storage().path;
        Self {
            inner: Vec::<PeerInfo>::new(),
            save_path: peer_storage_path.to_string(),
        }
    }

    fn init(&self) -> Self {
        let ct = fs::read_to_string(&self.save_path).unwrap();
        let mut remote_peers = Self::new();
        for str_peer in ct.split(",") {
            let peer_info: Vec<&str> = str_peer.split(",").collect();
            let name = peer_info.get(0).unwrap().to_string();
            let ip = peer_info.get(1).unwrap().to_string();
            let port: u32 = peer_info.get(2).unwrap().parse::<u32>().unwrap();
            let peer = PeerInfo::new(name, ip, port);

            remote_peers.inner.push(peer);
        }

        remote_peers
    }

    fn update(&self) {
        let mut ct = String::new();
        for peer_info in &self.inner {
            let str_peer_info = peer_info.name.to_string()
                + ","
                + &peer_info.ip
                + ","
                + &peer_info.port.to_string();
            ct += &str_peer_info
        }
        fs::write(&self.save_path, ct).unwrap();
    }

    fn insert(&mut self, name: String, ip: String, port: u32) {
        self.inner.push(PeerInfo {
            name: name,
            ip: ip,
            port: port,
        })
    }

    fn remove_by_name(&mut self, name: String) {
        let mut i = 0;
        let mut rm_index = 0;
        for peer_info in &self.inner {
            if peer_info.name == name {
                rm_index = i;
            }
            i += 1
        }
        self.inner.drain(rm_index..rm_index);
    }
}
