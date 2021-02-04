use crate::node::*;
use std::collections::HashMap;
use std::marker::PhantomData;


struct FnChain<In, Out, F> {
  f: F,
  _types: PhantomData<*const (In, Out)>,
}

impl<T, U, F> FnChain<T, U, F> where F: FnOnce(T) -> U {
  fn new(f: F) -> Self {
    Self {
      f,
      _types: PhantomData,
    }
  }

  fn chain<V, G>(self, g: G) -> FnChain<T, V, impl FnOnce(T) -> V> where G: FnOnce(U) -> V, {
    let f = self.f;
    FnChain {
      f: move |x| g(f(x)),
      _types: PhantomData,
    }
  }

  fn run(self, x: T) -> U {
    (self.f)(x)
  }
}

pub type Worker<'a> = dyn Fn<(&'a Node, InputData<'a>), Output = OutputData>;

pub struct Workers<'a> {
  map: HashMap<String, Box<Worker<'a>>>
}

impl <'a> Workers<'a> {

  pub fn new() -> Workers<'a> {
    Workers { map: HashMap::new() }
  }

  pub fn put(self: &mut Self, name: &str, worker: Box<Worker<'a>>) -> () {
    self.map.insert(name.to_string(), worker);
  }

  pub fn call(&self, name: &str, node: &'a Node, input: InputData<'a>) -> Option<OutputData> {
    self.map.get(name).map(|f| f(node, input))
  }
}