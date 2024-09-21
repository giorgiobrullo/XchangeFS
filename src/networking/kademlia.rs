use libp2p::kad::store::{MemoryStore, MemoryStoreConfig};
use libp2p::PeerId;

pub fn setup_kademlia(peer_id: PeerId) -> MemoryStore {
    // Add config
    let mut config = MemoryStoreConfig::default();
    config.max_records = 4096;
    config.max_provided_keys = 4096;

    let store = MemoryStore::with_config(peer_id, config);

    store
}
