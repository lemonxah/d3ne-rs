use crate::node::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkerError {
  #[error("Worker Not Found: `{0}`")]
  WorkerNotFound(String),
}

pub struct Workers(HashMap<String, Box<dyn Fn<(Node, InputData), Output = anyhow::Result<OutputData>>>>);

impl Workers {
  pub fn call(&self, name: &str, node: Node, input: InputData) -> anyhow::Result<OutputData> {
    self.0.get(name).map(|f| f(node, input)).ok_or(WorkerError::WorkerNotFound(name.into()))?
  }
}

pub struct WorkersBuilder {
  data: Vec<(String, Box<dyn Fn<(Node, InputData), Output = anyhow::Result<OutputData>>>)>
}

#[allow(dead_code)]
impl WorkersBuilder {
  pub fn new() -> WorkersBuilder {
    WorkersBuilder { data: vec![] }
  }

  pub fn add(&mut self, name: &str, worker: Box<dyn Fn<(Node, InputData), Output = anyhow::Result<OutputData>>>) -> &mut Self {
    self.data.push((name.to_string(), worker));
    self
  }

  pub fn build(self) -> Workers {
    Workers(self.data.into_iter().collect::<HashMap<_,_>>())
  }
}