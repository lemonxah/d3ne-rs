use crate::target::{Inputs, Outputs};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Number, Value};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug)]
pub struct IOData {
    pub data: Box<dyn Any>,
}

#[allow(dead_code)]
impl IOData {
    pub fn is<B: Any>(&self) -> bool {
        return TypeId::of::<B>() == (*self.data).type_id();
    }
    pub fn get<A>(&self) -> Option<&A>
    where
        A: 'static,
    {
        return self.data.downcast_ref::<A>();
    }
}

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Node input conversion error: {0}")]
    ConversionError(String),
    #[error("No value found for: {0}")]
    NoValueFound(String),
    #[error("Field: {0}, Value: {1}, Deserialization error: {2}")]
    DeserializeError(String, String, serde_json::Error),
}

#[derive(Debug)]
pub struct NodeResult(pub IOData);

impl Deref for NodeResult {
    type Target = IOData;
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
    data: Vec<(&'a str, Box<dyn Any>)>,
}

impl<'a> OutputDataBuilder<'a> {
    pub fn new() -> OutputDataBuilder<'a> {
        OutputDataBuilder { data: vec![] }
    }

    pub fn add_data(&mut self, key: &'a str, data: Box<dyn Any>) -> &mut Self {
        self.data.push((key, data));
        self
    }

    pub fn data(mut self, key: &'a str, data: Box<dyn Any>) -> Self {
        self.data.push((key, data));
        self
    }

    pub fn build(self) -> OutputData {
        OutputData(Rc::new(
            self.data
                .into_iter()
                .map(|(key, data)| (key.into(), NodeResult(IOData { data })))
                .collect::<HashMap<_, _>>(),
        ))
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
    data: Vec<(String, OutputData)>,
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
        InputData(
            self.data
                .into_iter()
                .map(|(key, data)| (key.into(), data))
                .collect::<HashMap<_, _>>(),
        )
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
    pub data: Option<Value>,
    pub group: Option<i64>,
    pub position: Option<Vec<f32>>,
    pub inputs: Option<Inputs>,
    pub outputs: Option<Outputs>,
}

impl Node {
    fn get_field<A>(
        &self,
        field: &'static str,
        inputs: &InputData,
        def: A,
        deref: Box<dyn Fn(&A) -> A>,
        convert: Box<dyn Fn(&Value) -> Result<A>>,
        noerr: Option<A>,
    ) -> Result<A>
    where
        A: 'static,
    {
        inputs
            .0
            .get(field)
            .and_then(|i| {
                i.get(
                    &self
                        .inputs
                        .clone()
                        .map(|i| i[field].connections[0].output.clone())
                        .unwrap_or_default(),
                )
            })
            .map(|v| Ok(v.get::<A>().map(deref).unwrap_or(def)))
            .or(self
                .data
                .clone()
                .and_then(|d| d.get(field).map(|v| convert(v))))
            .or(noerr.map(Ok))
            .unwrap_or(Err(anyhow!("{}", NodeError::NoValueFound(field.into()))))
    }

    // // TODO: Remove 'static from above method
    // pub fn get_field_t<T:Default + Clone + DeserializeOwned + 'static>(&self, field: &str, inputs: &InputData) -> Result<T> {
    //   self.get_field(field, inputs, T::default(), Box::new(|t| t.clone()), Box::new(|v| serde_json::from_value(v.clone()).unwrap()), None)
    // }

    pub fn get_number_field_or(
        &self,
        field: &'static str,
        inputs: &InputData,
        default: Option<i64>,
    ) -> Result<i64> {
        self.get_field(
            field,
            inputs,
            i64::MIN,
            Box::new(|r| *r),
            Box::new(|v| {
                v.as_i64().ok_or(anyhow!(NodeError::ConversionError(format!(
                    "Field: {}, Type: {}",
                    field.to_string(),
                    std::any::type_name::<i64>()
                ))))
            }),
            default,
        )
    }

    pub fn get_float_number_field_or(
        &self,
        field: &'static str,
        inputs: &InputData,
        default: Option<f64>,
    ) -> Result<f64> {
        self.get_field(
            field,
            inputs,
            f64::MIN,
            Box::new(|r| *r),
            Box::new(|v| {
                v.as_f64().ok_or(anyhow!(NodeError::ConversionError(format!(
                    "Field: {}, Type: {}",
                    field.to_string(),
                    std::any::type_name::<f64>()
                ))))
            }),
            default,
        )
    }

    pub fn get_string_field_or(
        &self,
        field: &'static str,
        inputs: &InputData,
        default: Option<String>,
    ) -> Result<String> {
        self.get_field(
            field,
            inputs,
            String::default(),
            Box::new(|r| r.to_owned()),
            Box::new(|v| {
                if let Value::String(s) = v {
                    Ok(s.clone())
                } else {
                    Err(anyhow!(NodeError::ConversionError(format!(
                        "Field: {}, Type: {}",
                        field.to_string(),
                        std::any::type_name::<String>()
                    ))))
                }
            }),
            default,
        )
    }

    pub fn get_json_field_or(
        &self,
        field: &'static str,
        inputs: &InputData,
        default: Option<Value>,
    ) -> Result<Value> {
        self.get_field(
            field,
            inputs,
            json!({}),
            Box::new(|r| r.clone()),
            Box::new(|v| {
                serde_json::from_str(v.as_str().ok_or(anyhow!(
                    "Field: {}, unable to get str value for deserialze",
                    field.to_string()
                ))?)
                .map_err(|e| {
                    anyhow!(NodeError::DeserializeError(
                        field.to_string(),
                        format!(
                            "[{:?}]json({})",
                            TypeId::of::<Value>(),
                            v.as_str().unwrap_or_default().to_string()
                        ),
                        e
                    ))
                })
            }),
            default,
        )
    }

    pub fn get_as_json_field_or(
        &self,
        field: &'static str,
        inputs: &InputData,
        default: Option<Value>,
    ) -> Result<Value> {
        let err = format!(
            "Field: {}, No josn, bool, i64, f64 or String value found",
            field.to_string()
        );
        inputs
            .get(field)
            .and_then(|i| {
                i.get(
                    &self
                        .inputs
                        .clone()
                        .map(|i| i[field].connections[0].output.clone())
                        .unwrap_or_default(),
                )
                .map(|r| {
                    let NodeResult(v) = r;
                    if v.is::<Value>() {
                        v.get::<Value>().map(|v| v.clone()).ok_or(anyhow!(
                            NodeError::ConversionError(
                                "Unable to get `Value` as json field".to_owned()
                            )
                        ))
                    } else if v.is::<bool>() {
                        v.get::<bool>().map(|b| Value::Bool(*b)).ok_or(anyhow!(
                            NodeError::ConversionError(
                                "Unable to get `bool` as json field".to_owned()
                            )
                        ))
                    } else if v.is::<i64>() {
                        v.get::<i64>()
                            .map(|i| Number::from(*i))
                            .map(Value::Number)
                            .ok_or(anyhow!(NodeError::ConversionError(
                                "Unable to get `i64` as json field".to_owned()
                            )))
                    } else if v.is::<f64>() {
                        v.get::<f64>()
                            .and_then(|f| Number::from_f64(*f))
                            .map(Value::Number)
                            .ok_or(anyhow!(NodeError::ConversionError(
                                "Unable to get `i64` as json field".to_owned()
                            )))
                    } else if v.is::<String>() {
                        v.get::<String>()
                            .map(|v| Value::String(v.clone()))
                            .ok_or(anyhow!(NodeError::ConversionError(
                                "Unable to get `String` as json field".to_owned()
                            )))
                    } else {
                        default.clone().ok_or(anyhow!(err.to_owned()))
                    }
                })
            })
            .or(self
                .data
                .clone()
                .and_then(|d| d.get(field).map(|v| Ok(v.clone()))))
            .or(default.map(Ok))
            .unwrap_or(Err(anyhow!(err.to_owned())))
    }

    pub fn get_as_json_field(&self, field: &'static str, inputs: &InputData) -> Result<Value> {
        self.get_as_json_field_or(field, inputs, None)
    }

    pub fn get_string_field(&self, field: &'static str, inputs: &InputData) -> Result<String> {
        self.get_string_field_or(field, inputs, None)
    }

    pub fn get_number_field(&self, field: &'static str, inputs: &InputData) -> Result<i64> {
        self.get_number_field_or(field, inputs, None)
    }

    pub fn get_float_number_field(&self, field: &'static str, inputs: &InputData) -> Result<f64> {
        self.get_float_number_field_or(field, inputs, None)
    }

    pub fn get_json_field(&self, field: &'static str, inputs: &InputData) -> Result<Value> {
        self.get_json_field_or(field, inputs, None)
    }
}
