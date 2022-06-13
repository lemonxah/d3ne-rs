use crate::node::*;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkerError {
  #[error("Worker Not Found: `{0}`")]
  WorkerNotFound(String),
  #[error("Node[{0}]: {1}")]
  NodeRunError(i64, anyhow::Error)
}

pub struct Workers<'a>(HashMap<String, Box<dyn Fn<(&'a Node, InputData), Output = anyhow::Result<OutputData>>>>);

impl <'a> Workers<'a> {
  pub fn call(&self, name: &str, node: &'a Node, input: InputData) -> anyhow::Result<OutputData> {
    self.0.get(name)
      .map(|f| 
        f(&node, input)
          .map_err(|e| 
            anyhow!(WorkerError::NodeRunError(node.id, e))
          )
        )
        .ok_or(WorkerError::WorkerNotFound(name.into()))?
  }
}

pub struct WorkersBuilder<'a> {
  data: Vec<(String, Box<dyn Fn<(&'a Node, InputData), Output = anyhow::Result<OutputData>>>)>
}

#[allow(dead_code)]
impl <'a> WorkersBuilder<'a> {
  pub fn new() -> WorkersBuilder<'a> {
    WorkersBuilder { data: vec![] }
  }

  pub fn add(&mut self, name: &str, worker: Box<dyn Fn<(&'a Node, InputData), Output = anyhow::Result<OutputData>>>) -> &mut Self {
    self.data.push((name.to_string(), worker));
    self
  }

  pub fn build(self) -> Workers<'a> {
    Workers(self.data.into_iter().collect::<HashMap<_,_>>())
  }
}