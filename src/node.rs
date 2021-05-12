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
    Ok(IOData {
      data: Box::new($data)
    })
  };
}
type Result<A> = std::result::Result<A, anyhow::Error>;

pub type NodeResult = Result<IOData>;
#[allow(dead_code)]
pub type OutputData = Rc<HashMap<String, NodeResult>>;
#[allow(dead_code)]
pub type InputData = HashMap<String, Rc<HashMap<String, NodeResult>>>;

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
  fn get_field<A>(&self, field: &str, inputs: &InputData, def: A, deref: Box<dyn Fn(&A) -> A>, convert: Box<dyn Fn(&Value) -> A> ) -> Result<A> where A: 'static {
    let v1: Option<Result<A>> = inputs.get(field)
      .and_then(|i| i.get(&self.inputs[field].connections[0].output))
      .map(|v| v.as_ref().map_err(|e| anyhow!(format!("{:?}", e))).map(|rv| rv.get::<A>().map(deref).unwrap_or(def)));
    match v1.or(self.data.get(field).map(|n| Ok(convert(n)))) {
      Some(v) => v,
      None => Err(anyhow!(format!("Node({}): no {:?} value found", &self.id, std::any::type_name::<A>())))
    }
  }
  
  pub fn get_number_field(&self, field: &str, inputs: &InputData) -> Result<i64> {
    self.get_field(field, inputs, i64::MIN, Box::new(|r| *r), Box::new(|v| v.as_i64().unwrap()))
  }
  
  pub fn get_float_number_field(&self, field: &str, inputs: &InputData) -> Result<f64> {
    self.get_field(field, inputs, f64::MIN, Box::new(|r| *r), Box::new(|v| v.as_f64().unwrap()))
  }

  pub fn get_string_field(&self, field: &str, inputs: &InputData) -> Result<String> {
    self.get_field(field, inputs, String::default(), Box::new(|r| r.clone()), Box::new(|n| if let Value::String(v) = n { v.clone() } else { "".to_string()}))
  }
  
  pub fn get_json_field(&self, field: &str, inputs: &InputData) -> Result<Value> {
    self.get_field(field, inputs, json!({}), Box::new(|r| r.clone()), Box::new(|n| serde_json::from_str(n.as_str().unwrap()).unwrap()))
  }

  pub fn get_as_json_field(&self, field: &str, inputs: &InputData) -> Result<Value> {
    let v1 = inputs.get(field).and_then(|i| i.get(&self.inputs[field].connections[0].output).map(|r| {
      match r {
        Ok(v) => if v.is::<Value>() {
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
          Err(anyhow!(format!("Node({}): no bool, i64, f64 or String value found", &self.id)))
        },
        Err(e) => Err(anyhow!(format!("{:?}",e)))
      }
    }));
    match v1.or(self.data.get(field).map(|v| Ok(v.clone()))) {
      Some(v) => v,
      None => Err(anyhow!(format!("Node({}): no bool, i64, f64 or String value found", &self.id)))
    }
  }

  pub fn get_as_json_field_or_default(&self, field: &str, inputs: &InputData) -> Result<Value> {
    let v1 = inputs.get(field).and_then(|i| i.get(&self.inputs[field].connections[0].output).map(|r| {
      match r {
        Ok(v) => if v.is::<Value>() {
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
          Ok(json!({}))
        },
        Err(e) => Err(anyhow!(format!("{:?}",e)))
      }
    }));
    match v1.or(self.data.get(field).map(|v| Ok(v.clone()))) {
      Some(v) => v,
      None => Ok(json!({}))
    }
  }

}