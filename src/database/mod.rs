pub mod database_manager;

use std::collections::{HashMap};

use serde::{Deserialize, Serialize};

use crate::table::Table;
use crate::error::{Result, NollaDBError};

use database_manager::DatabaseManager;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Database {
  pub database_name: String,
  pub tables: HashMap<String, Table>,
}

// use std::ops::{Deref, DerefMut};
// impl Deref for Database {
//     type Target = Database;
//     fn deref<'a>(&'a self) -> &'a Database {
//         &self
//     }
// }
// impl DerefMut for Database {
//     fn deref_mut<'a>(&'a mut self) -> &'a mut Database {
//         &mut self
//     }
// }

impl Database {
  pub fn new(database_name: String) -> Database {
    Database {
      database_name,
      tables: HashMap::new(),
    }
  }

  pub fn init(
    database_name: String,
    database_manager_file: String
  ) -> Result<(Database, DatabaseManager)> {
    let mut database = Database::new(database_name.clone());
    let mut database_manager = DatabaseManager::new();

    println!("reading {}...", database_name.clone());
    match Database::read(
      database_name.clone(),
      &Database::new(database_name.clone())
    ) {
      Ok(data) => {
        println!("database data {:#?}", data);
        database = data;
        println!("reading {} done", database_name);
        // 然后读 database_manager 文件
        match DatabaseManager::read(
          database_manager_file.clone(),
          &DatabaseManager::new(),
        ) {
          Ok(data) => {
            println!("database manager data {:#?}", data);
            database_manager = data;
            if !database_manager.has_database(database.database_name.clone()) {
              database_manager.database.insert(
                database.database_name.clone(),
                database.clone()
              );
            }
          },
          Err(error) => return Err(error),
        }
      },
      Err(error) => return Err(error),
    }

    Ok((database, database_manager))
  }

  pub fn open(
    database_manager: &DatabaseManager,
    database_name: String
  ) -> Result<&Self> {
    // TODO: 通过文件找到 database 路径
    // 目前先默认在当前目录
    match database_manager.get_database(database_name) {
      Ok(database) => Ok(database),
      Err(error) => return Err(error)
    }
  }

  pub fn read(database_name: String, new_data: &impl Serialize) -> Result<Self> {
    match DatabaseManager::read(
      database_name.clone(),
      new_data,
    ) {
      Ok(data) => Ok(data),
      Err(error) => return Err(error),
    }
  }

  pub fn save(database_name: String, data: &Database) -> Result<()> {
    match DatabaseManager::save(database_name, data) {
      Ok(()) => Ok(()),
      Err(error) => return Err(error),
    }
  }

  // 得到指定的 database 里的所有 table name
  pub fn get_all_tables(
    &self,
    database_manager: &DatabaseManager,
    database_name: String
  ) -> Result<Vec<String>> {
    let database = Database::open(&database_manager, database_name).unwrap();
    Ok(
      database.tables
        .iter()
        .map(|(key, _)| key.to_string())
        .collect()
    )
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


#[cfg(test)]
mod tests {
  use super::*;
  use std::result::Result;
  use rstest::rstest;
  use pretty_assertions::{assert_eq};
  use sqlparser::parser::Parser;
  use sqlparser::dialect::SQLiteDialect;
  use crate::sql_query::query::create::{CreateQuery};

  #[rstest]
  #[case("testdb")]
  fn test_create_new_database(
    #[case] database_name: &str,
  ) {
    assert_eq!(Database::new(database_name.to_string()).database_name, database_name);
  }

  #[rstest]
  #[case(
    "testdb",
    "CREATE TABLE test (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULl,
      email TEXT NOT NULL UNIQUE,
      active BOOL,
      score REAL
    );",
    "test"
  )]
  fn test_has_table(
    #[case] database_name: &str,
    #[case] query: &str,
    #[case] table_name: &str,
  ) {
    let database = create_new_database(database_name, query).unwrap();
    assert_eq!(database.has_table(table_name.to_string()), true);
  }

  #[rstest]
  #[case(
    "testdb",
    "CREATE TABLE test (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULl,
      email TEXT NOT NULL UNIQUE,
      active BOOL,
      score REAL
    );",
    "test",
    5,
    1,
  )]
  fn test_get_table(
    #[case] database_name: &str,
    #[case] query: &str,
    #[case] table_name: &str,
    #[case] expected_table_columns_len: usize,
    #[case] expected_most_recent_row_id: i64,
  ) {
    let database = create_new_database(database_name, query).unwrap();
    let mut database_mut = create_new_database(database_name, query).unwrap();

    let table = database.get_table(table_name.to_string()).unwrap();
    let mut table_mut = database_mut.get_table_mut(table_name.to_string()).unwrap();

    table_mut.most_recent_row_id += 1;

    assert_eq!(table.table_columns.len(), expected_table_columns_len);
    assert_eq!(table_mut.table_columns.len(), expected_table_columns_len);
    assert_eq!(table_mut.most_recent_row_id, expected_most_recent_row_id);
  }

  fn create_new_database(database_name: &str, query: &str) -> Result<Database, ()> {
    let mut database = Database::new(database_name.to_string());
    let dialect = SQLiteDialect {};
    let mut ast = Parser::parse_sql(&dialect, &query).unwrap();
    let create_query = CreateQuery::new(&ast.pop().unwrap()).unwrap();

    database.tables.insert(
      create_query.table_name.to_string(),
      Table::new(create_query),
    );

    Ok(database)
  }
}
