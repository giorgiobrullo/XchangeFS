use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::{identity::Keypair, PeerId, Swarm};
use std::error::Error;
use tracing::info;

use crate::networking::behavior::MyBehaviour;
use crate::networking::address::parse_listen_address;

use super::behavior::create_behaviour;

pub async fn build_swarm(
    listen_addr: Vec<String>,
    local_keypair: Keypair,
) -> Result<Swarm<MyBehaviour>, Box<dyn Error>> {
    let local_peer_id = PeerId::from(local_keypair.public());

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_keypair)
        .with_tokio()
        .with_quic()
        .with_behaviour(|key| {create_behaviour(&key).unwrap()}
        )?
        .build();

    info!("Local PeerId: {local_peer_id}");

    let mut is_listening = false;
    for addr in listen_addr {
        match parse_listen_address(&addr) {
            Ok(multiaddr) => {
                if swarm.listen_on(multiaddr.clone()).is_ok() {
                    info!("Listening on {multiaddr}");
                    is_listening = true;
                } else {
                    info!("Failed to listen on {multiaddr}");
                }
            }
            Err(err) => {
                info!("Error parsing address: {err}");
            }
        }
    }

    if !is_listening {
        return Err("Failed to listen on any address".into());
    }

    Ok(swarm)
}

pub async fn run_swarm(mut swarm: Swarm<MyBehaviour>) -> Result<(), Box<dyn Error>> {
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => info!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => info!("{event:?}"),
            _ => {}
        }
    }
}
