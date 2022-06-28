use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, ops::Deref};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputConnection {
    pub node: i64,
    pub output: String,
    pub data: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
    pub connections: Vec<InputConnection>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OutputConnection {
    pub node: i64,
    pub input: String,
    pub data: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Output {
    pub connections: Vec<OutputConnection>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Inputs(HashMap<String, Input>);

impl Inputs {
    pub fn inner(&self) -> &HashMap<String, Input> {
        &self.0
    }
}

impl Deref for Inputs {
    type Target = HashMap<String, Input>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Outputs(HashMap<String, Output>);

impl Outputs {
    pub fn inner(&self) -> &HashMap<String, Output> {
        &self.0
    }
}

impl Deref for Outputs {
    type Target = HashMap<String, Output>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
