use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use libp2p::identity::Keypair;
use libp2p::{noise, PeerId, Swarm, Transport, yamux};
use libp2p::core::upgrade;
use libp2p::gossipsub::{GossipsubConfigBuilder,IdentTopic as Topic, GossipsubMessage, MessageId, ValidationMode};
use libp2p::swarm::SwarmBuilder;
use libp2p::tcp::TokioTcpConfig;
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, Mutex};
use anyhow::Result;

use crate::networks::behaviour::BlockchainBehaviour;
use crate::networks::command::Messages;

static ID_KEYS: Lazy<Keypair> = Lazy::new(Keypair::generate_ed25519);
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(ID_KEYS.public()));
pub static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("block"));
pub static TRANX_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("tranxs"));
pub static WALLET_MAP: Lazy<Arc<Mutex<HashMap<String, String>>>>
= Lazy::new(||Arc::new(Mutex::new(HashMap::new())));

pub async fn create_swarm(
    topics: Vec<Topic>,
    msg_sender: mpsc::UnboundedSender<Messages>
) -> Result<Swarm<BlockchainBehaviour>> {
    println!("Local peer id: {:?}", &PEER_ID);
    let noise_keys
        = noise::Keypair::<noise::X25519Spec>::new().into_authentic(&ID_KEYS)?;

    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    let message_id_fn = |message : &GossipsubMessage| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        MessageId::from(s.finish().to_string())
    };

    let gossipsub_config = GossipsubConfigBuilder::default()
        .heartbeat_initial_delay(Duration::from_secs(10))
        .validation_mode(ValidationMode::Strict)
        .message_id_fn(message_id_fn)
        .build()
        .expect("Valid config");

    let mut behaviour = BlockchainBehaviour::new(
        ID_KEYS.clone(),
        gossipsub_config,
        msg_sender).await?;

    for topic in topics.iter() {
        behaviour.gossipsub.subscribe(topic).unwrap();
    }

    let swarm = SwarmBuilder::new(transport, behaviour, PEER_ID.clone())
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        })).build();

    Ok(swarm)
}
