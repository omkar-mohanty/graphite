use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub struct Node {
    id: Uuid,
    data: Box<dyn Serialize>
    
}

impl Node {
    pub fn new(data: impl Serialize ) -> Self {
        let id = Uuid::new_v4();
        Self { id, data }
    }
}
