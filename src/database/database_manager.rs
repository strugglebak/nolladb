use std::collections::{HashMap};
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use bincode::{deserialize, serialize};
use crate::error::{Result, NollaDBError};
use crate::database::Database;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct DatabaseManager {
  pub database: HashMap<String, Database>,
}
impl DatabaseManager {
  pub fn new() -> DatabaseManager {
    DatabaseManager { database: HashMap::new(), }
  }

  pub fn get_database(&self, database_name: String) -> Result<&Database> {
    match self.database.get(&database_name) {
      Some(database) => Ok(database),
      _ => Err(NollaDBError::General(String::from("Database not found"))),
    }
  }

  pub fn has_database(&self, database_name: String) -> bool {
    self.database.contains_key(&database_name)
  }

  // 从磁盘读取到内存
  pub fn read(filename: String) -> Result<Self> {
    // 先看 filename 在不在，不在就创建
    if let Err(_) = File::open(filename.clone()) {
      DatabaseManager::write_data(
        &filename.to_string(),
        &DatabaseManager::new()
      );
    }

    match
      DatabaseManager::read_data(&filename.to_string()) {
        Ok(data) => Ok(data),
        Err(error) => {
          return Err(error);
        },
    }
  }

  // 从内存写入到磁盘
  pub fn save(filename: String, data: DatabaseManager) -> Result<()> {
    DatabaseManager::write_data(
      &filename.to_string(),
      &data,
    );
    Ok(())
  }

  pub fn write_data(filename: &str, data: &impl Serialize) {
    let filename = format!("{}", filename);
    let bytes: Vec<u8> = serialize(&data).unwrap();
    let mut file = File::create(filename).unwrap();
    file.write_all(&bytes).unwrap();
  }

  pub fn read_data<T: DeserializeOwned>(filename: &str) -> Result<T> {
      let filename = format!("{}", filename);
      let mut file = File::open(filename).unwrap();
      let mut buffer = Vec::<u8>::new();
      file.read_to_end(&mut buffer).unwrap();
      let decoded: T = deserialize(&buffer[..]).unwrap();
      Ok(decoded)
  }
}
