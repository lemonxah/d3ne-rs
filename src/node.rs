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