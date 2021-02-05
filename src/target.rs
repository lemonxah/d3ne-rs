use std::collections::HashMap;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputConnection {
  pub node: i64,
  pub output: String,
  pub data: Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
  pub connections: Vec<InputConnection>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OutputConnection {
  pub node: i64,
  pub input: String,
  pub data: Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Output {
  pub connections: Vec<OutputConnection>,
}

pub type Inputs = HashMap<String, Input>;
pub type Outputs = HashMap<String, Output>;
