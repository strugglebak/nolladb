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

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
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
            Row::Text(BTreeMap::new())
          ),
        DataType::Bool => table_rows
          .clone()
          .borrow_mut()
          .insert(
            column_name.to_string(),
            Row::Bool(BTreeMap::new())
          ),
        DataType::Real => table_rows
          .clone()
          .borrow_mut()
          .insert(
            column_name.to_string(),
            Row::Real(BTreeMap::new())
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

  #[allow(dead_code)]
  pub fn get_column(&mut self, column_name: String) -> Result<&Column> {
    for table_column in self.table_columns.iter() {
      if table_column.column_name == column_name {
        return Ok(table_column);
      }
    }

    Err(NollaDBError::General(String::from("Column not found")))
  }

  pub fn get_column_mut(&mut self, column_name: String) -> Result<&mut Column> {
    // TODO: 待优化
    for table_column in self.table_columns.iter_mut() {
      if table_column.column_name == column_name {
        return Ok(table_column);
      }
    }

    Err(NollaDBError::General(String::from("Column not found")))
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

    if self.primary_key != "-1" {

      // 定义了某一列的名字为 PRIMARY KEY
      let mut table_certain_column_data =
        table_rows_data
          .get_mut(&self.primary_key)
          .unwrap();

      match
       table_column_names
        .iter()
        .any(|table_column_name| table_column_name == &self.primary_key) {
          // 如果要 INSERT 的列的列名包含 PRIMARY KEY
          // 就只更新 row id
          true => {

            if let Row::Integer(_) = &mut table_certain_column_data {
              for (i, table_column_name) in table_column_names.iter().enumerate() {
                // 循环并找到这一列
                if table_column_name != &self.primary_key { continue; }
                let value = &table_column_value[i];
                // 更新 row id
                new_row_id = value.parse::<i64>().unwrap();
              }
            }

          },
          // 如果要 INSERT 的列的列名不包含 PRIMARY KEY
          // 直接插入这一列的数据，不用更新 row id
          false => {

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

          },
      }
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
        Row::None => panic!("None column data found"),
      }
    }
    self.most_recent_row_id = new_row_id;
  }

  pub fn print_column_of_schema(&self) -> Result<usize> {
    let mut print_table = PrintTable::new();
    print_table.add_row(row![
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

      print_table.add_row(row![
        column_name,
        column_datatype,
        is_primary_key,
        is_unique_constraint,
        is_not_null_constraint,
        is_indexed,
      ]);
    }

    Ok(print_table.printstd())
  }

  pub fn print_table_data(&self) -> Result<usize> {
    let mut print_table = PrintTable::new();

    let column_names_vec = self
        .table_columns
        .iter()
        .map(|table_column| table_column.column_name.to_string())
        .collect::<Vec<String>>();

    // column name
    // 输出为最顶部的 header
    let print_table_rows_header = PrintRow::new(
      column_names_vec
        .iter()
        .map(|column_name| PrintCell::new(&column_name))
        .collect::<Vec<PrintCell>>(),
    );

    let table_rows_clone = Rc::clone(&self.table_rows);
    let table_rows_data =
      table_rows_clone
        .as_ref()
        .borrow();
    let table_first_column_data =
      table_rows_data
        .get(&self.table_columns.first().unwrap().column_name)
        .unwrap();

    let number_of_element_in_column
      = table_first_column_data.get_number_of_element_in_column();

    let mut print_table_rows: Vec<PrintRow>
      = vec![
        PrintRow::new(vec![]);
        number_of_element_in_column
      ];

    // 拿到每个 column_name 对应下的数据，并进行输出
    for column_name in &column_names_vec {
      let table_certain_column_data =
        table_rows_data
          .get(column_name)
          .expect("Can not find any rows with the given column name");
      let values_of_table_certain_column_data =
        table_certain_column_data.get_serialized_column_data();

      for i in 0..number_of_element_in_column {
        let mut cell_instance = "";
        if let Some(cell) =
          &values_of_table_certain_column_data.get(i) {
          cell_instance = cell;
        }
        print_table_rows[i].add_cell(PrintCell::new(cell_instance));
      }
    }

    print_table.add_row(print_table_rows_header);
    for row in print_table_rows {
      print_table.add_row(row);
    }

    Ok(print_table.printstd())
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

  #[rstest]
  #[case(DataType::Integer, "Integer")]
  #[case(DataType::Text, "Text")]
  #[case(DataType::Real, "Real")]
  #[case(DataType::Bool, "Boolean")]
  #[case(DataType::None, "None")]
  #[case(DataType::Invalid, "Invalid")]
  fn test_display_datatype(
    #[case] data_type: DataType,
    #[case] expected: &str,
  ) {
    assert_eq!(format!("{}", data_type), expected);
  }


  #[rstest]
  #[case(
    "CREATE TABLE test (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULl,
      email TEXT NOT NULL UNIQUE,
      active BOOL,
      score REAL
    );",
    5,
    0,
    true,
    DataType::Integer,
  )]
  fn test_create_new_table(
    #[case] query: &str,
    #[case] expected_table_columns_len: usize,
    #[case] expected_most_recent_row_id: i64,
    #[case] expected_is_primary_key: bool,
    #[case] expected_data_type: DataType,
  ) {
    let table = create_new_table(query).unwrap();

    if let Some(table_column) =
      table.table_columns
        .iter()
        .filter(|tc| tc.column_name == "id".to_string())
        .collect::<Vec<&Column>>()
        .first() {
      assert_eq!(table.table_columns.len(), expected_table_columns_len);
      assert_eq!(table.most_recent_row_id, expected_most_recent_row_id);
      assert_eq!(table_column.is_primary_key, expected_is_primary_key);
      assert_eq!(table_column.column_datatype, expected_data_type);
    };
  }

  #[rstest]
  #[case(
    "CREATE TABLE test (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULl,
      email TEXT NOT NULL UNIQUE,
      active BOOL,
      score REAL
    );",
    13,
  )]
  fn test_print_table_schema(
    #[case] query: &str,
    #[case] print_lines_number: usize
  ) {
    let table = create_new_table(query).unwrap();
    assert_eq!(table.print_column_of_schema(), Ok(print_lines_number));
  }

  fn create_new_table(query: &str) -> Result<Table, ()> {
    let dialect = SQLiteDialect {};
    let mut ast = Parser::parse_sql(&dialect, &query).unwrap();
    let create_query = CreateQuery::new(&ast.pop().unwrap()).unwrap();
    let table = Table::new(create_query);

    Ok(table)
  }
}
