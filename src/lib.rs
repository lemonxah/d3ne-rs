#![feature(unboxed_closures)]

#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;

pub mod target;
pub mod group;
#[macro_use] pub mod node;
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

    let engine = Engine::new("demo@0.1.1", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
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

    let engine = Engine::new("demo@0.1.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["num"].get::<i64>().unwrap();
    assert_eq!(result, &7i64);
  }

  #[test]
  fn branching_works_false() {
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
            "max": 5
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 3,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "true": {
              "connections": [{
                "node": 5,
                "input": "num",
                "data": {}
              }]
            },
            "false": {
              "connections": [{
                "node": 6,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [807.5, 228],
          "name": "Check"
        },
        "5": {
          "id": 5,
          "data": {
            "num2": 4
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 4,
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
        },
        "6": {
          "id": 6,
          "data": {
            "num2": 7
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 4,
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
    workers.put("Check", Box::new(check));
  
    let engine = Engine::new("demo@0.1.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["num"].get::<i64>().unwrap();
    assert_eq!(result, &10i64);
  }
  
  #[test]
  fn branching_works_true() {
    let json_data = r#"
    {
      "id": "demo@0.1.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": 6
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
            "max": 5
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 3,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "true": {
              "connections": [{
                "node": 5,
                "input": "num",
                "data": {}
              }]
            },
            "false": {
              "connections": [{
                "node": 6,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [807.5, 228],
          "name": "Check"
        },
        "5": {
          "id": 5,
          "data": {
            "num2": 4
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 4,
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
        },
        "6": {
          "id": 6,
          "data": {
            "num2": 7
          },
          "inputs": {
            "num": {
              "connections": [{
                "node": 4,
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
    workers.put("Check", Box::new(check));
  
    let engine = Engine::new("demo@0.1.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["num"].get::<i64>().unwrap();
    assert_eq!(result, &9i64);
  }
  
  fn check(node: Node, inputs: InputData) -> OutputData {
    let mut map = HashMap::new();
    let num = node.get_number_field("num", &inputs);
    let max = node.get_number_field("max", &inputs);
    if num.unwrap() > max.unwrap() {
      map.insert("true".to_string(), iodata!(max.unwrap()));
    } else {
      map.insert("false".to_string(), iodata!(num.unwrap()));
    }
    Rc::new(map)
  }

  fn number(node: Node, inputs: InputData) -> OutputData {
    let mut map = HashMap::new();
    let result = node.get_number_field("num", &inputs);
    map.insert("num".to_string(), IOData {
      data: Box::new(result.unwrap())
    });
    Rc::new(map)
  }

  fn add(node: Node, inputs: InputData) -> OutputData {
    let num = node.get_number_field("num", &inputs);
    let num2 = node.get_number_field("num2", &inputs);
    let mut map = HashMap::new();
    map.insert("num".to_string(), IOData {
      data: Box::new(num.unwrap() + num2.unwrap())
    });
    Rc::new(map)
  }

  fn multiply(node: Node, inputs: InputData) -> OutputData {
    let num = node.get_number_field("num", &inputs);
    let num2 = node.get_number_field("num2", &inputs);
    let mut map = HashMap::new();
    map.insert("num".to_string(), IOData {
      data: Box::new(num.unwrap() * num2.unwrap())
    });
    Rc::new(map)
  }
}
