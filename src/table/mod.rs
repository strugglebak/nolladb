mod row;
mod column;

use std::collections::{HashMap, BTreeMap};
use std::rc::Rc;
use std::cell::RefCell;

use serde::{Deserialize, Serialize};
use prettytable::{
  Table as PrintTable,
  Row as PrintRow,
  Cell as PrintCell,
};

use crate::sql_query::query::create::{
  CreateQuery,
  SchemaOfSQLColumn,
};
use crate::error::{Result, NollaDBError};

use row::Row;
use column::Column;
use column::data_type::DataType;
use column::index::Index;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Table {
  pub primary_key: String,
  pub table_name: String,
  pub indexes: HashMap<String, String>,
  pub most_recent_row_id: i64,
  pub table_rows: Rc<RefCell<HashMap<String, Row>>>,
  pub table_columns: Vec<Column>,
}

impl Table {
  // new 表示初始化一张 table，要初始化肯定是先创建
  pub fn new(create_query: CreateQuery) -> Self {
    // PRIMARY KEY 先默认 -1
    let mut primary_key: String = String::from("-1");

    let CreateQuery {
      table_name,
      table_metadata_columns,
    } = create_query;

    let indexes = HashMap::new();
    let most_recent_row_id = 0;

    // table rows 是由 RefCell 指针管理的 HashMap
    let table_rows: Rc<RefCell<HashMap<String, Row>>>
      = Rc::new(RefCell::new(HashMap::new()));
    // table columns 是 Column 元素组成的数组
    let mut table_columns: Vec<Column> = vec![];

    for table_metadata_column in &table_metadata_columns {
      let SchemaOfSQLColumn {
        column_name,
        column_datatype,
        is_primary_key,
        is_unique_constraint,
        is_not_null_constraint,
      } = &table_metadata_column;

      // 如果是 PRIMARY KEY，说明需要列名就是 PRIMARY KEY
      if *is_primary_key {
        primary_key = column_name.to_string();
      }

      // 构建 table rows
      match DataType::new(column_datatype.to_string()) {
        DataType::Integer => table_rows
          .clone()
          // 获取一个可变引用，配合 RefCell 使用
          .borrow_mut()
          .insert(
            column_name.to_string(),
            Row::Integer(BTreeMap::new())
          ),
        DataType::Text => table_rows
          .clone()
          .borrow_mut()
          .insert(
            column_name.to_string(),
            Row::Integer(BTreeMap::new())
          ),
        DataType::Bool => table_rows
          .clone()
          .borrow_mut()
          .insert(
            column_name.to_string(),
            Row::Integer(BTreeMap::new())
          ),
        DataType::Real => table_rows
          .clone()
          .borrow_mut()
          .insert(
            column_name.to_string(),
            Row::Integer(BTreeMap::new())
          ),
        DataType::None => table_rows
          .clone()
          .borrow_mut()
          .insert(
            column_name.to_string(),
            Row::None
          ),
        DataType::Invalid => table_rows
          .clone()
          .borrow_mut()
          .insert(
            column_name.to_string(),
            Row::None
          ),
      };

      // 构建 table columns
      table_columns.push(Column::new(
        column_name.to_string(),
        column_datatype.to_string(),
        *is_primary_key,
        *is_unique_constraint,
        *is_not_null_constraint,
      ));
    }

    Table {
      primary_key,
      table_name,
      indexes,
      most_recent_row_id,
      table_rows,
      table_columns,
    }
  }

  pub fn has_column(&self, column_name: String) -> bool {
    self
      .table_columns
      .iter()
      .any(|table_column| table_column.column_name == column_name)
  }

  pub fn get_column(&mut self, column_name: String) -> Result<&Column> {
    for table_column in self.table_columns.iter() {
      if table_column.column_name == column_name {
        return Ok(table_column);
      }
    }

    Err(NollaDBError::General(String::from("Column not found.")))
  }

  pub fn get_column_mut(&mut self, column_name: String) -> Result<&mut Column> {
    // TODO: 待优化
    for table_column in self.table_columns.iter_mut() {
      if table_column.column_name == column_name {
        return Ok(table_column);
      }
    }

    Err(NollaDBError::General(String::from("Column not found.")))
  }

  // 检查 InsertQuery 中的唯一性约束
  pub fn check_unique_constraint(
    &mut self,
    table_column_names: &Vec<String>,
    table_column_value: &Vec<String>,
  ) -> Result<()> {
    for (i, table_column_name) in table_column_names.iter().enumerate() {
      let table_column = self.get_column_mut(table_column_name.to_string()).unwrap();
      let Column { index, column_name, .. } = &table_column;

      // 找到下一个具备唯一性约束的 column 为止
      // 或者
      // 如果当前插入的 column name 和表中的 column name 不一致也继续找
      if !table_column.is_unique_constraint ||
         *table_column_name != *column_name { continue; }

      let column_value = &table_column_value[i];
      match index {
        Index::Integer(tree) => {
          if tree.contains_key(&column_value.parse::<i32>().unwrap()) {
            return Err(
              NollaDBError::General(
                format!(
                  "Error: column {} has a unique constraint violation,
                  value {} already exists for column {}",
                  *column_name, column_value, *column_name
                )
              )
            );
          }
        },
        Index::Text(tree) => {
          if tree.contains_key(column_value) {
            return Err(
              NollaDBError::General(
                format!(
                  "Error: column {} has a unique constraint violation,
                  value {} already exists for column {}",
                  *column_name, column_value, *column_name
                )
              )
            );
          }
        },
        Index::None => {
          return Err(
            NollaDBError::General(
              format!(
                "Error: cannot find index in column {} ",
                *column_name
              )
            )
          );
        },
      };
    }

    Ok(())
  }

  pub fn insert_row(
    &mut self,
    table_column_names: &Vec<String>,
    table_column_value: &Vec<String>,
  ) {
    let mut new_row_id = self.most_recent_row_id + i64::from(1);

    let table_rows_clone = Rc::clone(&self.table_rows);
    let mut table_rows_data =
      table_rows_clone
        .as_ref()
        .borrow_mut();
    let mut table_certain_column_data =
      table_rows_data
        .get_mut(&self.primary_key)
        .unwrap();

    match self.primary_key == "-1" {
      // 没设置 PRIMARY KEY
      true => {
        if let Row::Integer(_) = &mut table_certain_column_data {
          for (i, table_column_name) in table_column_names.iter().enumerate() {
            if table_column_name != &self.primary_key { continue; }
            let value = &table_column_value[i];
            new_row_id = value.parse::<i64>().unwrap();
          }
        }
      },
      // 设置了 PRIMARY KEY
      false => {
        if !table_column_names
          .iter()
          .any(|table_column_name| table_column_name == &self.primary_key) {

          if let Row::Integer(row_tree) = &mut table_certain_column_data {
            let key = new_row_id.clone();
            let value = new_row_id as i32;

            row_tree.insert(key, value);

            let table_certain_column_index =
              self
                .get_column_mut(self.primary_key.to_string())
                .unwrap()
                .get_index_mut();
            if let Index::Integer(column_tree) = table_certain_column_index {
              column_tree.insert(value, key);
            }

          }

        }

      },
    }

    // 检查 INSERT statement 中是否有表中没有的 column
    // 如果是，就对该 column 补 "Null"
    // 使得 row 的长度能对齐
    let mut j: usize = 0;
    let column_names_vec = self
        .table_columns
        .iter()
        .map(|table_column| table_column.column_name.to_string())
        .collect::<Vec<String>>();

    for i in 0..column_names_vec.len() {
      let key = &column_names_vec[i];
      let mut value = String::from("Null");

      match &table_column_names.get(j) {
        Some(table_column_name) => {
          if key == &table_column_name.to_string() {
            value = table_column_value[j].to_string();
            j += 1;
          } else {
            // 如果在原来的表中对应 column 名字不匹配
            // 如果是 PRIMARY KEY 则跳过
            if key == &self.primary_key { continue; }
          }
        },
        _ => {
          // 找到末尾
          // 如果是 PRIMARY KEY 则跳过
          if key == &self.primary_key { continue; }
        },
      }

      // 拿到这个 key 对应的 column data
      let mut table_key_corresponding_column_data =
        table_rows_data
          .get_mut(key)
          .unwrap();

      // 拿到这个 key 对应的 column index
      let table_key_corresponding_column_index =
        self
          .get_column_mut(key.to_string())
          .unwrap()
          .get_index_mut();

      // 更新
      let key = new_row_id.clone();
      match &mut table_key_corresponding_column_data {
        Row::Integer(row_tree) => {
          let value = value.parse::<i32>().unwrap();
          row_tree.insert(key, value);
          if let Index::Integer(column_tree) = table_key_corresponding_column_index {
            column_tree.insert(value, key);
          }
        },
        Row::Bool(row_tree) => {
          let value = value.parse::<bool>().unwrap();
          row_tree.insert(key, value);
        },
        Row::Text(row_tree) => {
          row_tree.insert(key, value.to_string());
          if let Index::Text(column_tree) = table_key_corresponding_column_index {
            column_tree.insert(value.to_string(), key);
          }
        },
        Row::Real(row_tree) => {
          let value = value.parse::<f32>().unwrap();
          row_tree.insert(key, value);
        },
        Row::None => panic!("None column data Found"),
      }
    }
    self.most_recent_row_id = new_row_id;
  }

  pub fn print_column_of_schema(&self) -> Result<usize> {
    let mut table = PrintTable::new();
    table.add_row(row![
      "Column Name",
      "Column DataType",
      "IS PRIMARY KEY",
      "IS UNIQUE",
      "IS NOT NULL",
      "IS INDEXED",
    ]);

    for table_column in &self.table_columns {
      let Column {
        column_name,
        column_datatype,
        is_primary_key,
        is_unique_constraint,
        is_not_null_constraint,
        is_indexed,
        ..
      } = &table_column;

      table.add_row(row![
        column_name,
        column_datatype,
        is_primary_key,
        is_unique_constraint,
        is_not_null_constraint,
        is_indexed,
      ]);
    }

    Ok(table.printstd())
  }
}
