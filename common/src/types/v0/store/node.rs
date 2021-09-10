//! Definition of node types that can be saved to the persistent store.

use crate::types::v0::{
    message_bus::{self, NodeId},
    openapi::models,
    store::{
        definitions::{ObjectKey, StorableObject, StorableObjectType},
        UuidString,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type NodeLabels = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Node {
    /// Node information.
    node: message_bus::NodeState,
    /// Node labels.
    labels: NodeLabels,
}

pub struct NodeState {
    /// Node information
    pub node: message_bus::NodeState,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct NodeSpec {
    /// Node identification.
    id: NodeId,
    /// Endpoint of the mayastor instance (gRPC)
    endpoint: String,
    /// Node labels.
    labels: NodeLabels,
}
impl NodeSpec {
    /// Return a new `Self`
    pub fn new(id: NodeId, endpoint: String, labels: NodeLabels) -> Self {
        Self {
            id,
            endpoint,
            labels,
        }
    }
    /// Node identification
    pub fn id(&self) -> &NodeId {
        &self.id
    }
    /// Node gRPC endpoint
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
    /// Node gRPC endpoint
    pub fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = endpoint
    }
}

impl From<NodeSpec> for models::NodeSpec {
    fn from(src: NodeSpec) -> Self {
        Self::new(src.endpoint, src.id)
    }
}
impl From<models::NodeSpec> for NodeSpec {
    fn from(src: models::NodeSpec) -> Self {
        Self::new(src.id.into(), src.grpc_endpoint, NodeLabels::new())
    }
}

impl UuidString for NodeSpec {
    fn uuid_as_string(&self) -> String {
        self.id.clone().into()
    }
}

/// Key used by the store to uniquely identify a NodeSpec structure.
pub struct NodeSpecKey(NodeId);

impl From<&NodeId> for NodeSpecKey {
    fn from(id: &NodeId) -> Self {
        Self(id.clone())
    }
}

impl ObjectKey for NodeSpecKey {
    fn key_type(&self) -> StorableObjectType {
        StorableObjectType::NodeSpec
    }

    fn key_uuid(&self) -> String {
        self.0.to_string()
    }
}

impl StorableObject for NodeSpec {
    type Key = NodeSpecKey;

    fn key(&self) -> Self::Key {
        NodeSpecKey(self.id.clone())
    }
}