use std::collections::{BTreeMap};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub enum Index {
  Integer(BTreeMap<i32, i64>),
  Text(BTreeMap<String, i64>),
  // Bool(BTreeMap<bool, i64>),
  // Real(BTreeMap<f32, i64>),
  None,
}
