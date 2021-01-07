use noise::{AuthenticKeypair, X25519};
use libp2p::{
    noise,
    identity::Keypair,
    PeerId
};

/// Stores auth details 
pub struct KeyStore {
    peer_id: PeerId,
    keys: AuthenticKeypair<X25519>
}

impl KeyStore {
    /// load keypair from file 
    pub fn new() -> Self {
        let id_keys = Keypair::generate_ed25519();
        let peer_id = PeerId::from_public_key(id_keys.public());

        // authenticate the keys with noise 
        let noise_keys = noise::Keypair::<noise::X25519>::new()
            .into_authentic(&id_keys)
            .expect("signing libp2p-noise static DH keypair failed");


            Self {
                keys: noise_keys,
                peer_id
            }
    }

    // gett the peer_id 
    pub fn get_id(&self) -> PeerId {
        self.peer_id.clone()
    }

    // get the authenticated keys 
    pub fn noise_keys(&self) -> AuthenticKeypair<X25519> {
        self.keys.clone()
    }


}