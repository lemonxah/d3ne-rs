
use serde_json::Value;
use std::collections::HashMap;
use crate::node::*;
use crate::target::*;
use crate::workers::Workers;

pub struct Engine<'a> {
  id: String,
  workers: Workers<'a>,
  nodes: HashMap<String, Node>,
  nodes_output: HashMap<i64, OutputData>,
}

impl <'a, 'b> Engine<'a> {
  pub fn new(id: &str, workers: Workers<'a>) -> Engine<'a> {
    Engine {
      id: id.to_string(),
      workers: workers,
      nodes: HashMap::new(),
      nodes_output: HashMap::new()
    }
  }

  pub fn process(self: &mut Self, json: &str) -> IOData {
    self.nodes.clear();
    let value: Value = serde_json::from_str(json).unwrap();
    dbg!(value.clone());
    self.nodes = serde_json::from_value(value["nodes"].clone()).unwrap();

    dbg!(&self.nodes);
    // let start_node = self.nodes.values().next().unwrap();
    // self.back_process(start_node);
    // self.forward_process(start_node);
    IOData { data: Box::new(0) }
  }

  fn back_process(&self, node: &'a Node) -> OutputData {
    if self.nodes_output.contains_key(&node.id) {
      // return self.nodes_output.get(&node.id).unwrap()
    }

    let mut input_data = InputData::new();
    for input in &node.inputs {
      for conn in &input.connections {
        // let out = self.back_process(&self.nodes[&conn.node]);
        // input_data.insert(input.name.clone(), out);
      }
    }

    // let output = self.workers.call(&node.name, node, input_data).unwrap();
    // let mut ms = self;
    // ms.nodes_output.insert(node.id, output); 
    // return self.nodes_output.get(&node.id).unwrap()
    let mut hm = HashMap::new();
    hm.insert("1".to_string(), IOData { data: Box::new(0) });
    hm
  }

  fn forward_process(&self, node: &'_ Node) {
    for output in &node.outputs {
      for connection in &output.connections {
        // self.back_process(&self.nodes[&connection.node]);
        // self.forward_process(&self.nodes[&connection.node]);
      }
    }
  }
}