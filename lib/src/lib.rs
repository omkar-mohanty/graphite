use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

pub struct Node<'a, R: NodeData<'a>> {
    _marker: &'a PhantomData<R>,
    data: R,
}

pub trait NodeData<'a>: Serialize + Deserialize<'a> {}

pub trait Edge<'a, V: NodeData<'a>> {
    fn get_vertices(&'a self) -> (Node<V>, Node<V>);
}

pub struct GraphDB {}
