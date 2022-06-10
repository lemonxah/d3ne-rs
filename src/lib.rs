#![feature(unboxed_closures)]

#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate anyhow;

mod target;
mod group;
#[macro_use] mod node;
mod workers;
mod engine;

pub use target::*;
pub use group::*;
pub use node::*;
pub use workers::*;
pub use engine::*;

#[cfg(test)]
mod tests {
  use crate::node::*;
  use crate::engine::Engine;
  use crate::workers::WorkersBuilder;
  use anyhow::Result;

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

    let mut workers = WorkersBuilder::new();
    workers.add("Number", Box::new(number))
      .add("Add", Box::new(add))
      .add("Multiply", Box::new(multiply));
    
    let engine = Engine::new("demo@0.1.1", workers.build());
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["num"].as_ref().unwrap().get::<i64>().unwrap();
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
    
    let mut workers = WorkersBuilder::new();

    workers.add("Number", Box::new(number));
    workers.add("Add", Box::new(add));

    let engine = Engine::new("demo@0.1.0", workers.build());
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["num"].as_ref().unwrap().get::<i64>().unwrap();
    assert_eq!(result, &7i64);
  }

  fn number(node: Node, inputs: InputData) -> Result<OutputData> {
    let result = node.get_number_field("num", &inputs)?;
    Ok(OutputDataBuilder::new()
      .add_data("num", Box::new(result))
      .build())
  }

  fn add(node: Node, inputs: InputData) -> Result<OutputData> {
    let num = node.get_number_field("num", &inputs)?;
    let num2 = node.get_number_field("num2", &inputs)?;
    Ok(OutputDataBuilder::new()
      .add_data("num", Box::new(num + num2))
      .build())
  }

  fn multiply(node: Node, inputs: InputData) -> Result<OutputData> {
    let num = node.get_number_field("num", &inputs)?;
    let num2 = node.get_number_field("num2", &inputs)?;
    Ok(OutputDataBuilder::new()
      .add_data("num", Box::new(num * num2))
      .build())
  }
}
