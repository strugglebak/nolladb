use std::collections::{BTreeMap};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum Row {
  Integer(BTreeMap<i64, i32>),
  Bool(BTreeMap<i64, bool>),
  Text(BTreeMap<i64, String>),
  Real(BTreeMap<i64, f32>),
  None,
}

// 这里是通过 Row 构建 Column
// 所以这里可以通过 Row 来推断出 Column 的情况
// Column 有一个 Index，这个 Index 也是由 BTreeMap 管理
// 这个 Index 里的 BTreeMap 存的 key/value 跟 Row 里面的 key/value 刚好相反
impl Row {
  pub fn get_number_of_element_in_column(&self) -> usize {
    match self {
      Row::Integer(tree) => tree.len(),
      Row::Bool(tree) => tree.len(),
      Row::Text(tree) => tree.len(),
      Row::Real(tree) => tree.len(),
      Row::None => panic!("Found None Type in columns"),
    }
  }

  pub fn get_serialized_column_data(&self) -> Vec<String> {
    match self {
      Row::Integer(tree) => tree.iter().map(|(_key, value)| value.to_string()).collect(),
      Row::Bool(tree) => tree.iter().map(|(_key, value)| value.to_string()).collect(),
      Row::Text(tree) => tree.iter().map(|(_key, value)| value.to_string()).collect(),
      Row::Real(tree) => tree.iter().map(|(_key, value)| value.to_string()).collect(),
      Row::None => panic!("Found None Type in columns"),
    }
  }
}
