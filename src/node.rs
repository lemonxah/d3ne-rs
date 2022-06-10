use std::ops::Deref;
use std::rc::Rc;
use std::any::{Any, TypeId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::target::{Inputs, Outputs};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug)]
pub struct IOData {
  pub data: Box<dyn Any>
}

#[allow(dead_code)]
impl IOData {
  pub fn is<B: Any>(&self) -> bool {
    return TypeId::of::<B>() == (*self.data).type_id()
  }
  pub fn get<A>(&self) -> Option<&A> where A: 'static {
    return self.data.downcast_ref::<A>();
  }
}

#[derive(Debug)]
pub struct NodeResult(pub Result<IOData>);

impl Deref for NodeResult {
    type Target = Result<IOData>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct OutputData(pub Rc<HashMap<String, NodeResult>>);

impl From<Rc<HashMap<String, NodeResult>>> for OutputData {
    fn from(inner: Rc<HashMap<String, NodeResult>>) -> Self {
        OutputData(inner)
    }
}

pub struct OutputDataBuilder<'a> {
  data: Vec<(&'a str, Box<dyn Any>)>
}

impl <'a> OutputDataBuilder<'a> {
  pub fn new() -> OutputDataBuilder<'a> {
    OutputDataBuilder { data: vec![] }
  }

  pub fn add_data(mut self, key: &'a str, data: Box<dyn Any>) -> OutputDataBuilder<'a> {
    self.data.push((key, data));
    self
  }

  pub fn build(self) -> OutputData {
    OutputData(Rc::new(self.data.into_iter().map(|(key, data)| (key.into(), NodeResult(Ok(IOData { data })))).collect::<HashMap<_,_>>() ))
  }

}

impl Deref for OutputData {
    type Target = Rc<HashMap<String, NodeResult>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct InputData(pub HashMap<String, OutputData>);

pub struct InputDataBuilder {
  data: Vec<(String, OutputData)>
}

impl InputDataBuilder {
  pub fn new() -> InputDataBuilder {
    InputDataBuilder { data: vec![] }
  }

  pub fn add_data(mut self, key: String, data: OutputData) -> InputDataBuilder {
    self.data.push((key, data));
    self
  }

  pub fn build(self) -> InputData {
    InputData(self.data.into_iter().map(|(key, data)| (key.into(), data)).collect::<HashMap<_,_>>())
  }
}

impl From<HashMap<String, OutputData>> for InputData {
    fn from(inner: HashMap<String, OutputData>) -> Self {
        InputData(inner)
    }
}

impl Deref for InputData {
  type Target = HashMap<String, OutputData>;
  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
  pub id: i64,
  pub name: String,
  pub data: Value,
  pub group: Option<i64>,
  pub position: Vec<f32>,
  pub inputs: Inputs,
  pub outputs: Outputs
}

impl Node {
  fn get_field<A>(&self, field: &str, inputs: &InputData, def: A, deref: Box<dyn Fn(&A) -> A>, convert: Box<dyn Fn(&Value) -> A>, noerr: Option<A> ) -> Result<A> where A: 'static {
    let v1: Option<Result<A>> = inputs.0.get(field)
      .and_then(|i| i.get(&self.inputs[field].connections[0].output))
      .map(|v| v.as_ref().map_err(|e| anyhow!(format!("{:?}", e))).map(|rv| rv.get::<A>().map(deref).unwrap_or(def)));
    match v1.or(self.data.get(field).map(|n| Ok(convert(n)))) {
      Some(v) => v,
      None => match noerr {
        None => Err(anyhow!(format!("Node({}): no {:?} value found", &self.id, std::any::type_name::<A>()))),
        Some(d) => Ok(d)
      }
    }
  }
  
  pub fn get_number_field_or(&self, field: &str, inputs: &InputData, default: Option<i64>) -> Result<i64> {
    self.get_field(field, inputs, i64::MIN, Box::new(|r| *r), Box::new(|v| v.as_i64().unwrap()), default)
  }
  
  pub fn get_float_number_field_or(&self, field: &str, inputs: &InputData, default: Option<f64>) -> Result<f64> {
    self.get_field(field, inputs, f64::MIN, Box::new(|r| *r), Box::new(|v| v.as_f64().unwrap()), default)
  }

  pub fn get_string_field_or(&self, field: &str, inputs: &InputData, default: Option<String>) -> Result<String> {
    self.get_field(field, inputs, String::default(), Box::new(|r| r.clone()), Box::new(|n| if let Value::String(v) = n { v.clone() } else { "".to_string()}), default)
  }

  pub fn get_json_field_or(&self, field: &str, inputs: &InputData, default: Option<Value>) -> Result<Value> {
    self.get_field(field, inputs, json!({}), Box::new(|r| r.clone()), Box::new(|n| serde_json::from_str(n.as_str().unwrap()).unwrap()), default)
  }

  pub fn get_as_json_field_or(&self, field: &str, inputs: &InputData, default: Option<Value>) -> Result<Value> {
    let v1 = inputs.get(field).and_then(|i| i.get(&self.inputs[field].connections[0].output).map(|r| {
      match r {
        NodeResult(Ok(v)) => if v.is::<Value>() {
          Ok((*v.get::<Value>().unwrap()).clone())
        } else if v.is::<bool>() {
          Ok(serde_json::from_str(&v.get::<bool>().unwrap().to_string()).unwrap())
        } else if v.is::<i64>() {
          Ok(serde_json::from_str(&v.get::<i64>().unwrap().to_string()).unwrap())
        } else if v.is::<f64>() {
          Ok(serde_json::from_str(&v.get::<f64>().unwrap().to_string()).unwrap())
        } else if v.is::<String>() {
          Ok(Value::String(v.get::<String>().unwrap().clone()))
        } else {
          default.clone().ok_or(anyhow!(format!("Node({}): no bool, i64, f64 or String value found", &self.id)))
        },
        NodeResult(Err(e)) => Err(anyhow!(format!("{:?}",e)))
      }
    }));
    match v1.or(self.data.get(field).map(|v| Ok(v.clone()))) {
      Some(v) => v,
      None => default.clone().ok_or(anyhow!(format!("Node({}): no bool, i64, f64 or String value found", &self.id)))
    }
  }

  pub fn get_as_json_field(&self, field: &str, inputs: &InputData) -> Result<Value> {
    self.get_as_json_field_or(field, inputs, None)
  }

  pub fn get_string_field(&self, field: &str, inputs: &InputData) -> Result<String> {
    self.get_string_field_or(field, inputs, None)
  }
  
  pub fn get_number_field(&self, field: &str, inputs: &InputData) -> Result<i64> {
    self.get_number_field_or(field, inputs, None)
  }
  
  pub fn get_float_number_field(&self, field: &str, inputs: &InputData) -> Result<f64> {
    self.get_float_number_field_or(field, inputs, None)
  }
  
  pub fn get_json_field(&self, field: &str, inputs: &InputData) -> Result<Value> {
    self.get_json_field_or(field, inputs, None)
  }
}