use std::rc::Rc;
use std::any::{Any, TypeId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::target::{Inputs, Outputs};
use std::collections::HashMap;

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

#[macro_export ]macro_rules! iodata {
  ($data: expr) => {
    IOData {
      data: Box::new($data)
    }
  };
}

#[allow(dead_code)]
pub type OutputData = Rc<HashMap<String, IOData>>;
#[allow(dead_code)]
pub type InputData = HashMap<String, Rc<HashMap<String, IOData>>>;

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
  pub fn get_number_field(&self, field: &str, inputs: &InputData) -> Option<i64> {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
    v1.or(self.data.get(field).map(|n| n.as_i64().unwrap()))
  }
  
  pub fn get_float_number_field(&self, field: &str, inputs: &InputData) -> Option<f64> {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| *v.get::<f64>().unwrap()).unwrap());
    v1.or(self.data.get(field).map(|n| n.as_f64().unwrap()))
  }
  
  pub fn get_string_field(&self, field: &str, inputs: &InputData) -> Option<String> {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| v.get::<String>().unwrap().clone()).unwrap());
    v1.or(self.data.get(field).map(|n| n.to_string()))
  }
  
  pub fn get_json_field(&self, field: &str, inputs: &InputData) -> Option<Value> {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| (v.get::<Value>()).unwrap().clone()).unwrap());
    v1.or(self.data.get(field).map(|n| serde_json::from_str(n.as_str().unwrap()).unwrap()))
  }

  pub fn get_as_json_field(&self, field: &str, inputs: &InputData) -> Option<Value> {
    let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| {
      if v.is::<Value>() {
        (*v.get::<Value>().unwrap()).clone()
      } else if v.is::<bool>() {
        serde_json::from_str(&v.get::<bool>().unwrap().to_string()).unwrap()
      } else if v.is::<i64>() {
        serde_json::from_str(&v.get::<i64>().unwrap().to_string()).unwrap()
      } else if v.is::<f64>() {
        serde_json::from_str(&v.get::<f64>().unwrap().to_string()).unwrap()
      } else if v.is::<String>() {
        serde_json::from_str(&v.get::<String>().unwrap()).unwrap()
      } else {
        json!({})
      }
    }).unwrap());
    v1.or(self.data.get(field).map(|v| v.clone()))
  }

}