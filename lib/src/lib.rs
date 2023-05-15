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
}

pub enum Direction {
    Inbound,
    Outbound,
}

pub struct Edge<'a> {
    tag: String,
    vertex1:&'a Node,
    vertex2:&'a Node,
    direction: Option<Direction>
}

impl<'a> Edge<'a> {
    pub fn new(vertex1:&'a Node, vertex2:&'a Node, tag: String) -> Self {
        Edge { vertex1, vertex2, direction: None, tag }
    }
}

pub struct Database<'a> {
    nodes: BTreeMap<Uuid, Node>,
    edges: BTreeSet<Edge<'a>>
}

impl<'a> Database<'a> {
    pub fn new() -> Self {
        Database {
            nodes: BTreeMap::new(),
            edges: BTreeSet::new(),
        }
    }
}
