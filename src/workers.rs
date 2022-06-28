use crate::node::*;
use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkerError {
    #[error("Worker Not Found: `{0}`")]
    WorkerNotFound(String),
    #[error("Node[{0}]: {1}")]
    NodeRunError(i64, anyhow::Error),
}

pub trait Worker {
    fn name(&self) -> &str;
    fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData>;
}

pub struct Workers(HashMap<String, Box<dyn Worker>>);

impl Workers {
    pub fn call(&self, name: &str, node: &Node, input: InputData) -> Result<OutputData> {
        self.0
            .get(name)
            .map(|worker| {
                worker
                    .work(&node, input)
                    .map_err(|e| anyhow!(WorkerError::NodeRunError(node.id, e)))
            })
            .ok_or(WorkerError::WorkerNotFound(name.into()))?
    }
}

pub struct WorkersBuilder {
    data: Vec<(String, Box<dyn Worker>)>,
}

#[allow(dead_code)]
impl WorkersBuilder {
    pub fn new() -> WorkersBuilder {
        WorkersBuilder { data: vec![] }
    }

    pub fn add<A>(&mut self, worker: A) -> &mut Self
    where
        A: Worker + 'static,
    {
        self.data
            .push((worker.name().to_string(), Box::new(worker)));
        self
    }

    pub fn build(self) -> Workers {
        Workers(self.data.into_iter().collect::<HashMap<_, _>>())
    }
}
