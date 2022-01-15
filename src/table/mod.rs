mod row;
mod column;

use std::collections::{HashMap, BTreeMap};
use std::rc::Rc;
use std::cell::RefCell;

use serde::{Deserialize, Serialize};

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

  pub fn get_column_mut<'a>(&mut self, column_name: String) -> Result<&mut Column> {
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
    for (i, table_column_name) in
      table_column_names
      .iter()
      .enumerate() {
      let table_column = self.get_column_mut(table_column_name.to_string()).unwrap();
      let Column { index, column_name, .. } = &table_column;

      // 找到下一个具备唯一性约束的 column 为止
      // 或者
      // 如果当前插入的 column name 和表中的 column name 不一致也继续找
      if !table_column.is_unique_constraint ||
         *table_column_name != *column_name {
        continue;
      }

      let column_value = &table_column_value[i];
      match index {
        Index::Integer(tree) => {
          if tree.contains_key(&column_value.parse::<i64>().unwrap()) {
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
          if tree.contains_key(&column_value.parse::<i64>().unwrap()) {
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

  }
}
