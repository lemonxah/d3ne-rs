use std::rc::Rc;
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
    let mut closed_nodes: Vec<i64> = Vec::new();
    let end_id = self.process_nodes(&nodes[&start_node_id], &nodes, &mut cache, &mut closed_nodes);
    Ok(cache[&end_id].clone())
  }

  fn process_node(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, cache: &mut HashMap<i64, OutputData>, closed_nodes: &mut Vec<i64>) -> OutputData {
    if cache.contains_key(&node.id) {
      return cache[&node.id].clone();
    }
    println!("current node: {}, node type: {}, disabled nodes: {:?}", node.id, node.name, &closed_nodes);    
    if closed_nodes.contains(&node.id) {
      return Rc::new(HashMap::new());
    }

    let mut input_data = InputData::new();
    for (name, input) in &node.inputs {
      for conn in &input.connections {
        if !closed_nodes.contains(&conn.node) {
          let out = self.process_node(&nodes[&conn.node], nodes, cache, closed_nodes);
          println!("node: {}, contains key: {}, input connection: {}, output: {:?}", &conn.node, out.clone().contains_key(&conn.output), &conn.output, &out);
          if out.clone().contains_key(&conn.output) {
            input_data.insert(name.clone(), out);
          } else if name != "action" {
            println!("should close node: {}", &conn.node);
            self.disable_node_tree(&nodes[&conn.node], nodes, closed_nodes);
            self.disable_node_tree(node, nodes, closed_nodes);
          } else {
            println!("should only be actions: {}", name);
          }
        } else {
          println!("not running for input connection: {:?}", conn);
        }
      }
    }
    let mut output = Rc::new(HashMap::new());
    if !closed_nodes.contains(&node.id) {
      output = self.workers.call(&node.name, node.clone(), input_data).unwrap();
      cache.insert(node.id, output.clone());
    }
    return output;
  }

  fn process_nodes(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, cache: &mut HashMap<i64, OutputData>, closed_nodes: &mut Vec<i64>) -> i64 {
    let mut id: i64 = node.id;
    if !closed_nodes.contains(&node.id) {
      let outputdata = self.process_node(&node, &nodes, cache, closed_nodes);
      for (name, output) in &node.outputs {
        if outputdata.contains_key(name) {
          for connection in &output.connections {
            if !closed_nodes.contains(&connection.node) {
              id = self.process_nodes(&nodes[&connection.node], &nodes, cache, closed_nodes);
            }
          }
        } else {
          if name != "action" {
            println!("disabling connections for output: {}", name);
            for connection in &output.connections {
              if connection.input == "action" {
                self.disable_node_tree(&nodes[&connection.node], nodes, closed_nodes);
              }
            }
          }
        }
      }
    }
    id
  }

  fn disable_node_tree(&self, node: &'_ Node, nodes: &HashMap<i64, Node>, closed_nodes: &mut Vec<i64>) {
    match node.inputs.get("action") {
      None => (),
      Some(input) => {
        if input.connections.len() == 1 {
          closed_nodes.push(node.id);
          println!("node disabled: {}", node.id);
          for (_, output) in node.outputs.clone() {
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