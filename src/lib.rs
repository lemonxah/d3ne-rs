#![feature(unboxed_closures)]

#[macro_use] extern crate serde;
extern crate serde_json;

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
  use std::rc::Rc;
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
    
    let mut workers = Workers::new();

    workers.put("Number", Box::new(number));
    workers.put("Add", Box::new(add));
    let mut engine = Engine::new("demo@0.1.0", workers);
    let output = engine.process(json_data);
    println!("output: {:?}", output);
    let oo = output.unwrap();
    let result = oo["num"].get::<i64>().unwrap();
    assert_eq!(result, &7i64);
  }

  fn number(node: Node, _inputs: InputData) -> OutputData {
    let mut map = HashMap::new();
    let result = node.data["num"].to_string().parse::<i64>().unwrap();
    map.insert("num".to_string(), IOData {
      data: Box::new(result)
    });
    println!("sourcing: {:?}", result);
    Rc::new(map)
  }

  fn add(node: Node, inputs: InputData) -> OutputData {
    let num = node.data.get("num").map(|n| n.as_i64().unwrap()).unwrap_or(
      inputs.get("num").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap()).unwrap()
    );
    let num2 = node.data.get("num2").map(|n| n.as_i64().unwrap()).unwrap_or({
      let n2 = inputs.get("num2").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
      n2.unwrap_or(0)
    });

    println!("adding: {:?} + {:?} = {:?}", num, num2, num + num2);

    let mut map = HashMap::new();
    map.insert("num".to_string(), IOData {
      data: Box::new(num + num2)
    });
    Rc::new(map)
  }

}
