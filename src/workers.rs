use crate::node::*;
use std::collections::HashMap;

pub struct Workers {
  map: HashMap<String, Box<dyn Fn<(Node, InputData), Output = OutputData>>>
}

#[allow(dead_code)]
impl Workers {
  pub fn new() -> Workers {
    Workers { map: HashMap::new() }
  }

  pub fn put(self: &mut Self, name: &str, worker: Box<dyn Fn<(Node, InputData), Output = OutputData>>) -> () {
    self.map.insert(name.to_string(), worker);
  }


}

pub trait CallableWorkers {
  fn call(&self, name: &str, node: Node, input: InputData) -> Option<OutputData>;
}

impl CallableWorkers for Workers{
  fn call(&self, name: &str, node: Node, input: InputData) -> Option<OutputData> {
    self.map.get(name).map(|f| f(node, input))
  }
}
