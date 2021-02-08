#![feature(unboxed_closures)]

#[macro_use] extern crate serde;
extern crate serde_json;

pub mod target;
pub mod group;
pub mod node;
pub mod workers;
pub mod engine;

#[cfg(test)]
mod tests {
  use crate::node::*;
  use crate::engine::Engine;
  use crate::workers::Workers;
  use std::collections::HashMap;
  use std::rc::Rc;

  #[test]
  fn multiply_works() {
    let json_data = r#"
    {
      "id": "demo@0.1.1",
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
              }, {
                "node": 4,
                "input": "num2",
                "data": {}
              }, {
                "node": 5,
                "input": "num2",
                "data": {}
              }]
            }
          },
          "position": [-60, 182],
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
          "position": [-106, 378],
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
          "position": [241, 240],
          "name": "Add"
        },
        "4": {
          "id": 4,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 3,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                "node": 5,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [552.5, 204],
          "name": "Add"
        },
        "5": {
          "id": 5,
          "data": {},
          "inputs": {
            "num": {
              "connections": [{
                "node": 4,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": []
            }
          },
          "position": [826.5, 292],
          "name": "Multiply"
        }
      },
      "comments": []
    }
    "#;

    let mut workers = Workers::new();

    workers.put("Number", Box::new(number));
    workers.put("Add", Box::new(add));
    workers.put("Multiply", Box::new(multiply));

    let mut engine = Engine::new("demo@0.1.1", workers);
    let output = engine.process(json_data);
    println!("output: {:?}", output);
    let oo = output.unwrap();
    let result = oo["num"].get::<i64>().unwrap();
    assert_eq!(result, &8i64);

  }

  #[test]
  fn add_works() {
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
              }, {
                "node": 4,
                "input": "num2",
                "data": {}
              }, {
                "node": 5,
                "input": "num2",
                "data": {}
              }]
            }
          },
          "position": [-98, 218],
          "name": "Number"
        },
        "2": {
          "id": 2,
          "data": {
            "num": 1
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
          "position": [-147, 406],
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
          "position": [424, 238],
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
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                "node": 5,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [807.5, 228],
          "name": "Add"
        },
        "5": {
          "id": 5,
          "data": {
            "num2": 0
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 4,
                "output": "num",
                "data": {}
              }]
            },
            "num2": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": []
            }
          },
          "position": [1084.5, 243],
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
    let inum1 = inputs.get("num").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
    let num = inum1.or(node.data.get("num").map(|n| n.as_i64().unwrap())).unwrap();

    let inum2 = inputs.get("num2").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
    let num2 = inum2.or(node.data.get("num2").map(|n| n.as_i64().unwrap())).unwrap();

    println!("adding: {:?} + {:?} = {:?}", num, num2, num + num2);

    let mut map = HashMap::new();
    map.insert("num".to_string(), IOData {
      data: Box::new(num + num2)
    });
    Rc::new(map)
  }

  fn multiply(node: Node, inputs: InputData) -> OutputData {
    let inum1 = inputs.get("num").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
    let num = inum1.or(node.data.get("num").map(|n| n.as_i64().unwrap())).unwrap();

    let inum2 = inputs.get("num2").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
    let num2 = inum2.or(node.data.get("num2").map(|n| n.as_i64().unwrap())).unwrap();

    println!("multiplying: {:?} * {:?} = {:?}", num, num2, num * num2);

    let mut map = HashMap::new();
    map.insert("num".to_string(), IOData {
      data: Box::new(num * num2)
    });
    Rc::new(map)
  }

}
