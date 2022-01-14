use std::collections::{BTreeMap};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum Index {
  Integer(BTreeMap<i64, i32>),
  // Bool(BTreeMap<i64, bool>),
  Text(BTreeMap<i64, String>),
  // Real(BTreeMap<i64, f32>),
  None,
}
