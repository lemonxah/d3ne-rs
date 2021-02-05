use serde_json::Value;
use std::collections::HashMap;
use crate::node::*;
use crate::workers::Workers;

pub struct Engine {
  id: String,
  workers: Workers,
}

#[derive(Debug)]
pub enum Error {
  VersionMismatch,
}

impl <'a, 'b> Engine {
  pub fn new(id: &str, workers: Workers) -> Engine {
    Engine {
      id: id.to_string(),
      workers: workers,
    }
  }

  pub fn process(&mut self, json: &str) -> Result<OutputData, Error> {
    let value: Value = serde_json::from_str(json).unwrap();
    let version = value["id"].as_str().unwrap().to_string();
    if self.id != version {
      return Err(Error::VersionMismatch);
    }
    let nodess: HashMap<String, Node> = serde_json::from_value(value["nodes"].clone()).unwrap();
    let nodes: HashMap<i64, Node> = nodess.into_iter().map(|(k,v)| (k.parse::<i64>().unwrap(), v)).collect();

    let mut cache: HashMap<i64, OutputData> = HashMap::new();
    let start_node = nodes.values().next().unwrap();
    self.back_process(start_node, &nodes, &mut cache);
    self.forward_process(start_node, &nodes, &mut cache);
    // dbg!(&start_node);
    let end_node = nodes.values().into_iter().find(|n| n.outputs.len() == 0 || n.outputs.clone().into_iter().all(|(_k,v)| v.connections.len() == 0)).unwrap();
    // dbg!(end_node);
    Ok(cache[&end_node.id].clone())
  }

  fn back_process(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, cache: &mut HashMap<i64, OutputData>) -> OutputData {
    if cache.contains_key(&node.id) {
      return cache[&node.id].clone();
    }

    let mut input_data = InputData::new();
    for (name, input) in &node.inputs {
      for conn in &input.connections {
        let out = self.back_process(&nodes[&conn.node], nodes, cache);
        input_data.insert(name.clone(), out);
      }
    }
    // dbg!(&input_data, &node.name, &node.id);
    let output = self.workers.call(&node.name, node.clone(), input_data).unwrap();
    cache.insert(node.id, output.clone());
    // dbg!(&cache);
    return cache[&node.id].clone();
  }

  fn forward_process(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, cache: &mut HashMap<i64, OutputData>) {
    for (_name, output) in &node.outputs {
      for connection in &output.connections {
        self.back_process(&nodes[&connection.node], &nodes, cache);
        self.forward_process(&nodes[&connection.node], &nodes, cache);
      }
    }
  }
}