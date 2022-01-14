mod index;
mod data_type;

use std::collections::{BTreeMap};

use serde::{Deserialize, Serialize};

use index::Index;
use data_type::DataType;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Column {
  pub column_name: String,
  pub column_datatype: DataType,
  pub is_primary_key: bool,
  pub is_unique_constraint: bool,
  pub is_not_null_constraint: bool,
  pub is_indexed: bool,
  pub index: Index,
}

impl Column {
  pub fn new(
    column_name: String,
    column_datatype: DataType,
    is_primary_key: bool,
    is_unique_constraint: bool,
    is_not_null_constraint: bool,
  ) -> Self {
    let cd = DataType::new(column_datatype.to_string());
    let index = match column_datatype {
      DataType::Integer => Index::Integer(BTreeMap::new()),
      DataType::Bool => Index::None,
      DataType::Text => Index::Text(BTreeMap::new()),
      DataType::Real => Index::None,
      DataType::None => Index::None,
      DataType::Invalid => Index::None,
    };

    Column {
      column_name,
      column_datatype: cd,
      is_primary_key,
      is_unique_constraint,
      is_not_null_constraint,
      is_indexed: is_primary_key,
      index,
    }
  }

  pub fn get_mut_index(&mut self) -> &mut Index {
    &mut self.index
  }
}
