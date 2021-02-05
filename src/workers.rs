use crate::node::*;
use std::collections::HashMap;
use std::marker::PhantomData;


// struct FnChain<In, Out, F> {
//   f: F,
//   _types: PhantomData<*const (In, Out)>,
// }

// impl<T, U, F> FnChain<T, U, F> where F: FnOnce(T) -> U {
//   fn new(f: F) -> Self {
//     Self {
//       f,
//       _types: PhantomData,
//     }
//   }

//   fn chain<V, G>(self, g: G) -> FnChain<T, V, impl FnOnce(T) -> V> where G: FnOnce(U) -> V, {
//     let f = self.f;
//     FnChain {
//       f: move |x| g(f(x)),
//       _types: PhantomData,
//     }
//   }

//   fn run(self, x: T) -> U {
//     (self.f)(x)
//   }
// }

pub struct Workers {
  map: HashMap<String, Box<dyn Fn<(Node, InputData), Output = OutputData>>>
}

impl Workers {

  pub fn new() -> Workers {
    Workers { map: HashMap::new() }
  }

  pub fn put(self: &mut Self, name: &str, worker: Box<dyn Fn<(Node, InputData), Output = OutputData>>) -> () {
    self.map.insert(name.to_string(), worker);
  }

  pub fn call(&self, name: &str, node: Node, input: InputData) -> Option<OutputData> {
    self.map.get(name).map(|f| f(node, input))
  }
}