use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputConnection {
  pub node: String,
  pub output: String,
  pub data: Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
  pub name: String,
  pub connections: Vec<InputConnection>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OutputConnection {
  pub node: String,
  pub input: String,
  pub data: Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Output {
  pub name: String,
  pub connections: Vec<OutputConnection>,
}

pub type Inputs = Vec<Input>;
pub type Outputs = Vec<Output>;
