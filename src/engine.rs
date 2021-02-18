use serde_json::Value;
use std::collections::HashMap;
use crate::node::*;
use crate::workers::Workers;

#[allow(dead_code)]
pub struct Engine {
  id: String,
  workers: Workers,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
  VersionMismatch,
}

#[allow(dead_code)]
impl <'a, 'b> Engine {
  pub fn new(id: &str, workers: Workers) -> Engine {
    Engine {
      id: id.to_string(),
      workers: workers,
    }
  }

  pub fn parse_json(&self, json: &str) -> Result<HashMap<i64, Node>, Error> {
    let value: Value = serde_json::from_str(json).unwrap();
    self.parse_value(value)
  }

  pub fn parse_value(&self, value: Value) -> Result<HashMap<i64, Node>, Error> {
    let version = value["id"].as_str().unwrap().to_string();
    if self.id != version {
      return Err(Error::VersionMismatch);
    }
    let nodess: HashMap<String, Node> = serde_json::from_value(value["nodes"].clone()).unwrap();
    Ok(nodess.into_iter().map(|(k,v)| (k.parse::<i64>().unwrap(), v)).collect())
  }

  pub fn process(&self, nodes: &HashMap<i64, Node>, start_node_id: i64) -> Result<OutputData, Error> {
    let mut cache: HashMap<i64, OutputData> = HashMap::new();
    let end_id = self.process_nodes(&nodes[&start_node_id], &nodes, &mut cache);
    Ok(cache[&end_id].clone())
  }

  fn process_node(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, cache: &mut HashMap<i64, OutputData>) -> OutputData {
    if cache.contains_key(&node.id) {
      return cache[&node.id].clone();
    }

    let mut input_data = InputData::new();
    for (name, input) in &node.inputs {
      for conn in &input.connections {
        let out = self.process_node(&nodes[&conn.node], nodes, cache);
        input_data.insert(name.clone(), out);
      }
    }
    let output = self.workers.call(&node.name, node.clone(), input_data).unwrap();
    cache.insert(node.id, output.clone());
    return output;
  }

  fn process_nodes(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, cache: &mut HashMap<i64, OutputData>) -> i64 {
    let outputdata = self.process_node(&node, &nodes, cache);
    let mut id: i64 = node.id;
    for (name, output) in &node.outputs {
      if outputdata.contains_key(name) {
        for connection in &output.connections {
          id = self.process_nodes(&nodes[&connection.node], &nodes, cache);
        }
      }
    }
    id
  }
}