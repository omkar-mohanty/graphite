use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

pub struct Node {
    id: Uuid,
    data: Vec<u8>,
}

impl Node {
    pub fn new(data: &[u8]) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            data: data.to_vec(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}

pub enum Direction {
    Inbound,
    Outbound,
}

pub struct Edge {
    tag: String,
    vertex1: Uuid,
    vertex2: Uuid,
    direction: Option<Direction>,
}

impl Edge {
    pub fn new<'a>(vertex1: Uuid, vertex2: Uuid, tag: String) -> Self {
        Edge {
            vertex1,
            vertex2,
            direction: None,
            tag,
        }
    }
}

pub struct Database {
    nodes: BTreeMap<Uuid, Node>,
    edges: BTreeMap<Uuid, Edge>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            nodes: BTreeMap::new(),
            edges: BTreeMap::new(),
        }
    }
}
