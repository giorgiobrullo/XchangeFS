use libp2p::swarm::NetworkBehaviour;
use libp2p::{autonat, identify, kad, mdns, ping};
use libp2p::kad::store::MemoryStore;
use libp2p::{PeerId, identity::Keypair};
use std::error::Error;

#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub ping: ping::Behaviour,
    pub kademlia: kad::Behaviour<MemoryStore>,
    pub mdns: mdns::async_io::Behaviour,
    pub identity: identify::Behaviour,
    pub autonat: autonat::Behaviour
}

pub fn create_behaviour(
    local_keypair: &Keypair
) -> Result<MyBehaviour, Box<dyn Error>> {
    let local_peer_id = PeerId::from(local_keypair.public());

    let ping_behavior = ping::Behaviour::default();
    let kademlia = kad::Behaviour::new(local_peer_id, MemoryStore::new(local_peer_id));
    let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id)?;
    let identity = identify::Behaviour::new(identify::Config::new("xchangefs/0.0.1".to_string(), local_keypair.public()));
    let autonat = autonat::Behaviour::new(local_peer_id, autonat::Config::default());


    Ok(MyBehaviour {
        ping: ping_behavior,
        kademlia,
        mdns,
        identity,
        autonat
    })
}
