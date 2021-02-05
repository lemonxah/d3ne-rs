use std::any::{Any, TypeId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::target::{Inputs, Outputs};
use std::collections::HashMap;

pub struct IOData {
  pub data: Box<dyn Any>
}

impl IOData {
  fn is<B: Any>(&self) -> bool {
    return TypeId::of::<B>() == (*self.data).type_id()
  }
  fn get<A>(&self) -> Option<&A> where A: 'static {
    return self.data.downcast_ref::<A>();
  }
}

pub type OutputData = HashMap<String, IOData>;
pub type InputData<'a> = HashMap<String, &'a HashMap<String, IOData>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
  pub id: i64,
  pub name: String,
  pub data: Value,
  pub group: i64,
  pub position: Vec<f32>,
  pub inputs: Inputs,
  pub outputs: Outputs
}

pub type Nodes = HashMap<String, Node>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct D3NE {
  pub id: String,
  pub nodes: Nodes,
  pub comments: Vec<String>,
}