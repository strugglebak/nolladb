
use std::collections::{HashMap};
use serde::{Deserialize, Serialize};
use crate::error::{Result, NollaDBError};
use crate::database::Database;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct DatabaseMap {
  pub database: HashMap<String, Database>,
}
impl DatabaseMap {
  pub fn new(database: Database) -> Self {
    DatabaseMap {
      database: HashMap::new(),
    }
  }

  pub fn get_database(&self, database_name: String) -> Result<&Database> {
    match self.database.get(&database_name) {
      Some(database) => Ok(database),
      _ => Err(NollaDBError::General(String::from("Database not found"))),
    }
  }
}
