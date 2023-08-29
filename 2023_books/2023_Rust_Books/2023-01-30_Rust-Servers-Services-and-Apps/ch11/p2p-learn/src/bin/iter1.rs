use libp2p::{identity, PeerId};

#[tokio::main]
async fn main() {
	let key = identity::Keypair::generate_ed25519();
	let peer_id = PeerId::from(key.public());
	
	println!("New peer id: {:?}", peer_id);
	
	// /ip4/192.158.1.23/tcp/1234/p2p/12D3KooW...
}
