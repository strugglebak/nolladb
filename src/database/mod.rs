use std::collections::{HashMap};

use serde::{Deserialize, Serialize};

use crate::table::Table;
use crate::error::{Result, NollaDBError};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Database {
  pub database_name: String,
  pub tables: HashMap<String, Table>,
}

impl Database {
  pub fn new(database_name: String) -> Self {
    Database {
      database_name,
      tables: HashMap::new(),
    }
  }

  pub fn has_table(&self, table_name: String) -> bool {
    self.tables.contains_key(&table_name)
  }

  pub fn get_table(&self, table_name: String) -> Result<&Table> {
    match self.tables.get(&table_name) {
      Some(table) => Ok(table),
      _ => Err(NollaDBError::General(String::from("Table not found"))),
    }
  }

  pub fn get_table_mut(&mut self, table_name: String) -> Result<&mut Table> {
    match self.tables.get_mut(&table_name) {
      Some(table) => Ok(table),
      _ => Err(NollaDBError::General(String::from("Table not found"))),
    }
  }
}
