use std::collections::HashMap;

pub struct Group {
  id: i64,
  nodes: Vec<i64>,
  min_width: f32,
  max_widht: f32,
  position: [f32; 2],
  width: f32,
  height: f32
}

pub type Groups = HashMap<i64, Group>;