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

pub struct Database {
    nodes: BTreeMap<Uuid, Node>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            nodes: BTreeMap::new(),
        }
    }
}

struct Vector {

}
