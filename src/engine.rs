use std::rc::Rc;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;
use crate::{node::*, WorkerError};
use crate::workers::Workers;


#[derive(Error, Debug)]
pub enum EngineError {
  #[error("Version mismatch: Engine({0}), Nodes({1})")]
  VersionMismatch(String, String),
  #[error(transparent)]
  WorkerError(WorkerError),
  #[error(transparent)]
  Other(#[from] anyhow::Error),
}

#[allow(dead_code)]
pub struct Engine {
  id: String,
  workers: Workers,
}

#[allow(dead_code)]
impl <'a, 'b> Engine {
  pub fn new(id: &str, workers: Workers) -> Engine {
    Engine {
      id: id.to_string(),
      workers: workers,
    }
  }

  pub fn parse_json(&self, json: &str) -> Result<HashMap<i64, Node>, EngineError> {
    let value: Value = serde_json::from_str(json).unwrap();
    self.parse_value(value)
  }

  pub fn parse_value(&self, value: Value) -> Result<HashMap<i64, Node>, EngineError> {
    let version = value["id"].as_str().unwrap().to_string();
    if self.id != version {
      return Err(EngineError::VersionMismatch(self.id.clone(), version));
    }
    let nodess: HashMap<String, Node> = serde_json::from_value(value["nodes"].clone()).unwrap();
    Ok(nodess.into_iter().map(|(k,v)| (k.parse::<i64>().unwrap(), v)).collect())
  }

  pub fn process(&self, nodes: &HashMap<i64, Node>, start_node_id: i64) -> Result<OutputData, EngineError> {
    let mut cache: HashMap<i64, OutputData> = HashMap::new();
    let mut closed_nodes: Vec<i64> = Vec::new();
    let end_id = self.process_nodes(&nodes[&start_node_id], &nodes, &mut cache, &mut closed_nodes)?;
    Ok(cache[&end_id].clone().into())
  }

  fn process_node(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, cache: &mut HashMap<i64, OutputData>, closed_nodes: &mut Vec<i64>) -> Result<OutputData, EngineError> {
    if cache.contains_key(&node.id) {
      return Ok(cache[&node.id].clone().into());
    }
    if closed_nodes.contains(&node.id) {
      return Ok(Rc::new(HashMap::new()).into());
    }

    let mut input_data: Vec<(String, OutputData)> = vec![];
    for (name, input) in node.inputs.inner() {
      for conn in &input.connections {
        if !closed_nodes.contains(&conn.node) {
          let out = self.process_node(&nodes[&conn.node], nodes, cache, closed_nodes)?;
          input_data.push((name.clone(), out.clone().into()));
          if !out.clone().contains_key(&conn.output) {
            if conn.output != "action" {
              self.disable_node_tree(&nodes[&conn.node], nodes, closed_nodes);
              self.disable_node_tree(node, nodes, closed_nodes);
            }
          }
        }
      }
    }
    let mut output = Rc::new(HashMap::new()).into();
    if !closed_nodes.contains(&node.id) {
      output = self.workers.call(
        &node.name, node.clone(),
        input_data.into_iter().fold(InputDataBuilder::new(), |b, (key, data)| {
          b.add_data(key, data)
        }).build()
      ).map_err(|e| EngineError::WorkerError(e))?;
      cache.insert(node.id, output.clone().into());
    }
    return Ok(output);
  }

  fn process_nodes(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, cache: &mut HashMap<i64, OutputData>, closed_nodes: &mut Vec<i64>) -> Result<i64, EngineError> {
    let mut id: i64 = node.id;
    if !closed_nodes.contains(&node.id) {
      let outputdata = self.process_node(&node, &nodes, cache, closed_nodes)?;
      for (name, output) in node.outputs.inner() {
        if outputdata.contains_key(name) {
          for connection in &output.connections {
            if !closed_nodes.contains(&connection.node) {
              id = self.process_nodes(&nodes[&connection.node], &nodes, cache, closed_nodes)?;
            }
          }
        } else {
          if name != "action" {
            for connection in &output.connections {
              if connection.input == name.clone() && !closed_nodes.contains(&connection.node) {
                self.disable_node_tree(&nodes[&connection.node], nodes, closed_nodes);
              }
            }
          }
        }
      }
    }
    Ok(id)
  }

  fn disable_node_tree(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, closed_nodes: &mut Vec<i64>) {
    match node.inputs.get("action") {
      None => (),
      Some(input) => {
        if input.connections.len() == 1 {
          if !closed_nodes.contains(&node.id) {
            // println!("node disabled: {}", node.id);
            closed_nodes.push(node.id);
          }
          for (_, output) in node.outputs.clone().inner() {
            for connection in &output.connections {
              let _node = &nodes[&connection.node];
              match _node.inputs.get("action") {
                None => (),
                Some(input) => {
                  if let Some(_) = input.connections.clone().into_iter().find(|c| c.node == connection.node) {
                    self.disable_node_tree(&nodes[&connection.node], nodes, closed_nodes);
                  }
                }
              }
            }
          }
        }
      },
    }
  }
}