use libp2p::identity::{
    secp256k1::{Keypair, SecretKey},
    PublicKey,
};
use libp2p::PeerId;

pub fn generate_secret() -> (SecretKey, PeerId) {
    let keypair = Keypair::generate();
    let public_key = PublicKey::Secp256k1(keypair.public().clone());
    let peer_id = PeerId::from_public_key(public_key);
    (keypair.into(), peer_id)
}
