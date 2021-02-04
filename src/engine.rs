use json::JsonValue;
use std::collections::HashMap;
use crate::node::*;
use crate::target::*;
use crate::workers::Workers;

pub struct Engine<'a> {
  id: String,
  workers: Workers<'a>,
  nodes: Nodes,
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

  pub fn process(self: &mut Self, json: JsonValue) -> IOData {
    self.nodes.clear();
    self.nodes = json["nodes"].entries().map(|(k,v)| {
      (k.to_string().parse().unwrap(), Node {
        id: k.to_string().parse().unwrap(),
        name: v["name"].to_string(),
        position: v["position"].members().map(|v| v.to_string().parse::<f32>().unwrap()).collect(),
        data: v["data"].clone(),
        group: 0,
        inputs: v["inputs"].entries().map(|(k,v)| {
          Input {
            name: k.to_string(),
            connections: v["connections"].members().map(|v| {
              InputConnection {
                node: v["node"].to_string().parse().unwrap(),
                output: v["output"].to_string(),
                data: v["data"].clone(),
              }
            }).collect()
          }
        }).collect::<Vec<_>>(),
        outputs: v["inputs"].entries().map(|(k,v)| {
          Output {
            name: k.to_string(),
            connections: v["connections"].members().map(|v| {
              OutputConnection {
                node: v["node"].to_string().parse().unwrap(),
                input: v["input"].to_string(),
                data: v["data"].clone(),
              }
            }).collect()
          }
        }).collect::<Vec<_>>(),
      })
    }).collect();

    let start_node = self.nodes.values().next().unwrap();
    self.back_process(start_node);
    self.forward_process(start_node);
    IOData { data: Box::new(0) }
  }

  fn back_process(&self, node: &'_ Node) -> &OutputData {
    if self.nodes_output.contains_key(&node.id) {
      return self.nodes_output.get(&node.id).unwrap()
    }

    let mut input_data = InputData::new();
    for input in &node.inputs {
      for conn in &input.connections {
        let out = self.back_process(&self.nodes[&conn.node]);
        input_data.insert(input.name.clone(), out);
      }
    }

    let output = self.workers.call(&node.name, node, input_data).unwrap();
    let mut ms = self;
    ms.nodes_output.insert(node.id, output); 
    return self.nodes_output.get(&node.id).unwrap()
  }

  fn forward_process(&self, node: &'_ Node) {
    for output in &node.outputs {
      for connection in &output.connections {
        self.back_process(&self.nodes[&connection.node]);
        self.forward_process(&self.nodes[&connection.node]);
      }
    }
  }
}