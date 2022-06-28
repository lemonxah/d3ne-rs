#[macro_use]
extern crate anyhow;

mod group;
mod target;
#[macro_use]
mod node;
mod engine;
mod workers;

pub use engine::*;
pub use group::*;
pub use node::*;
pub use target::*;
pub use workers::*;

#[cfg(test)]
mod tests {
    use crate::engine::Engine;
    use crate::workers::WorkersBuilder;
    use crate::{node::*, Worker, WorkerError};
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
        workers.add(Number).add(Add).add(Multiply);

        let engine = Engine::new("demo@0.1.1", workers.build());
        let nodes = engine.parse_json(json_data).unwrap();
        let nn = nodes.clone();
        let output = engine.process(&nn, 1);
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

        let mut workers = WorkersBuilder::new();

        workers.add(Number);
        workers.add(Add);

        let engine = Engine::new("demo@0.1.0", workers.build());
        let nodes = engine.parse_json(json_data).unwrap();
        let output = engine.process(&nodes, 1);
        let oo = output.unwrap();
        let result = oo["num"].get::<i64>().unwrap();
        assert_eq!(result, &7i64);
    }

    #[test]
    fn errors_propegate() {
        let json_data = r#"
    {
      "id": "demo@0.1.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": "abc"
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
          "position": [807.5, 228],
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
          "position": [1084.5, 243],
          "name": "Add"
        }
      },
      "comments": []
    }
    "#;

        let mut workers = WorkersBuilder::new();

        workers.add(Number);
        workers.add(Add);

        let engine = Engine::new("demo@0.1.0", workers.build());
        let nodes = engine.parse_json(json_data).unwrap();
        let output = engine.process(&nodes, 1);
        // Node[1]: Node input conversion error: Field: num, Type: i64
        let err: anyhow::Error = output.err().unwrap();
        let expected: anyhow::Error = anyhow!(WorkerError::NodeRunError(
            1,
            anyhow!(NodeError::ConversionError(
                "Field: num, Type: i64".to_owned()
            ))
        ));
        println!("{:?}", &err);
        println!("{:?}", &expected);
        assert_eq!(err.to_string(), expected.to_string());
    }

    struct Number;
    impl Worker for Number {
        fn name(&self) -> &str {
            "Number"
        }

        fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
            let result = node.get_number_field("num", &input_data)?;
            Ok(OutputDataBuilder::new()
                .data("num", Box::new(result))
                .build())
        }
    }

    struct Add;
    impl Worker for Add {
        fn name(&self) -> &str {
            "Add"
        }

        fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
            let num = node.get_number_field("num", &input_data)?;
            let num2 = node.get_number_field("num2", &input_data)?;
            Ok(OutputDataBuilder::new()
                .data("num", Box::new(num + num2))
                .build())
        }
    }

    struct Multiply;
    impl Worker for Multiply {
        fn name(&self) -> &str {
            "Multiply"
        }

        fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
            let num = node.get_number_field("num", &input_data)?;
            let num2 = node.get_number_field("num2", &input_data)?;
            Ok(OutputDataBuilder::new()
                .data("num", Box::new(num * num2))
                .build())
        }
    }
}
