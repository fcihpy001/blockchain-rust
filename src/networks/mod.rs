use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc};
use std::time::Duration;
use libp2p::gossipsub::{GossipsubConfigBuilder, GossipsubMessage, IdentTopic as Topic, MessageId, ValidationMode};
use libp2p::identity::Keypair;
use libp2p::{noise, PeerId, Swarm, Transport, yamux};
use libp2p::core::upgrade;
use libp2p::swarm::SwarmBuilder;
use libp2p::tcp::TokioTcpConfig;
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, Mutex};
use crate::networks::behaviour::BlockchainBehaviour;
use crate::networks::command::Messages;
use anyhow::Result;


pub mod behaviour;
pub mod command;
pub mod node;
pub mod swarm;


