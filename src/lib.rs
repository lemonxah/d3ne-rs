#![feature(unboxed_closures)]

extern crate json;

mod target;
mod group;
mod node;
mod workers;
mod engine;

#[cfg(test)]
mod tests {
  use crate::node::*;
  use crate::engine::Engine;
  use crate::workers::Workers;
  use std::collections::HashMap;
  #[test]
  fn it_works() {
    let json_data = r#"
    {
      "id": "demo@0.1.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": 2
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 3,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [80, 200],
          "name": "Number"
        },
        "2": {
          "id": 2,
          "data": {
            "num": 0
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 3,
                "input": "num2",
                "data": {}
              }]
            }
          },
          "position": [80, 400],
          "name": "Number"
        },
        "3": {
          "id": 3,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 2,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                "node": 4,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [390, 216],
          "name": "Add"
        },
        "4": {
          "id": 4,
          "data": {
            "num2": 5
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 3,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": []
            }
          },
          "outputs": {
            "num": {
              "connections": []
            }
          },
          "position": [693.5, 225],
          "name": "Add"
        }
      },
      "comments": []
    }
    "#;
    let json = json::parse(json_data).unwrap();
    let mut workers = Workers::new();

    workers.put("Number", Box::new(number));
    workers.put("Add", Box::new(add));
    let mut engine = Engine::new("rete@1.0.0", workers);
    let output = engine.process(json);
    assert_eq!(output.data.downcast_ref::<i32>().unwrap(), &7);
  }

  fn number(node: &Node, _inputs: InputData) -> OutputData {
    let mut map = HashMap::new();
    map.insert("num".to_string(), IOData {
      data: Box::new(node.data["num"].to_string().parse::<i32>().unwrap())
    });
    map
  }

  fn add(node: &Node, _inputs: InputData) -> OutputData {
    let num = node.clone().inputs.into_iter().find(|v| v.name == "num").map(|v| v.connections[0].data.to_string()).unwrap_or(node.data["num"].to_string()).parse::<i32>().unwrap();
    let num2 = node.clone().inputs.into_iter().find(|v| v.name == "num2").map(|v| v.connections[0].data.to_string()).unwrap_or(node.data["num2"].to_string()).parse::<i32>().unwrap();
    let mut map = HashMap::new();
    map.insert("num".to_string(), IOData {
      data: Box::new(num + num2)
    });
    map
  }


}
