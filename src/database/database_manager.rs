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

  pub fn get_database_mut(&mut self, database_name: String) -> Result<&mut Database> {
    match self.database.get_mut(&database_name) {
      Some(database) => Ok(database),
      _ => Err(NollaDBError::General(String::from("Database not found"))),
    }
  }

  pub fn has_database(&self, database_name: String) -> bool {
    self.database.contains_key(&database_name)
  }

  // 从磁盘读取到内存
  pub fn read<T: DeserializeOwned>(filename: String, new_data: &impl Serialize) -> Result<T> {
    // 先看 filename 在不在，不在就创建这个 file
    if let Err(_) = File::open(filename.clone()) {
      println!("{} creating...", filename);
      DatabaseManager::write_data(
        &filename.to_string(),
        new_data,
      );
      println!("creating {} done", filename);
    }

    match
      DatabaseManager::read_data(&filename.to_string()) {
        Ok(data) => Ok(data),
        Err(error) => return Err(error),
    }
  }

  // 从内存写入到磁盘
  pub fn save(filename: String, data: &impl Serialize) -> Result<()> {
    DatabaseManager::write_data(
      &filename.to_string(),
      data,
    );
    Ok(())
  }

  fn write_data(filename: &str, data: &impl Serialize) {
    let filename = format!("{}", filename);
    let bytes: Vec<u8> = serialize(&data).unwrap();
    let mut file = File::create(filename).unwrap();
    file.write_all(&bytes).unwrap();
  }

  fn read_data<T: DeserializeOwned>(filename: &str) -> Result<T> {
      let filename = format!("{}", filename);
      let mut file = File::open(filename).unwrap();
      let mut buffer = Vec::<u8>::new();
      file.read_to_end(&mut buffer).unwrap();
      let decoded: T = deserialize(&buffer[..]).unwrap();
      Ok(decoded)
  }
}
