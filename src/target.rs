use json::JsonValue;

#[derive(Clone)]
pub struct InputConnection {
  pub node: i64,
  pub output: String,
  pub data: JsonValue
}

#[derive(Clone)]
pub struct Input {
  pub name: String,
  pub connections: Vec<InputConnection>,
}

#[derive(Clone)]
pub struct OutputConnection {
  pub node: i64,
  pub input: String,
  pub data: JsonValue
}

#[derive(Clone)]
pub struct Output {
  pub name: String,
  pub connections: Vec<OutputConnection>,
}

pub type Inputs = Vec<Input>;
pub type Outputs = Vec<Output>;
