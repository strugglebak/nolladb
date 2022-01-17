use sqlparser::ast::{Statement, DataType, ColumnOption};
use crate::error::{Result, NollaDBError};

#[derive(Debug, PartialEq)]
// TODO: 待优化
pub struct SchemaOfSQLColumn {
  pub column_name: String,
  pub column_datatype: String,
  pub is_primary_key: bool,
  pub is_unique_constraint: bool,
  pub is_not_null_constraint: bool,
}

#[derive(Debug)]
pub struct CreateQuery {
  pub table_name: String,
  pub table_metadata_columns: Vec<SchemaOfSQLColumn>,
}

impl CreateQuery {
  pub fn new(statement: &Statement) -> Result<CreateQuery> {
    #[allow(unused_assignments)]
    let mut option_table_name: Option<String> = None;
    let mut table_metadata_columns: Vec<SchemaOfSQLColumn> = vec![];

    match statement {
      Statement::CreateTable {
        name,
        columns,
        constraints,
        // with_options,
        // external,
        // file_format,
        // location,
        ..
      } => {
        option_table_name = Some(name.to_string());

        // 处理 columns
        for column in columns {
          let column_name = column.name.to_string();

          // 先检查在创建表时有没有插入相同名字的 column
          if table_metadata_columns
              .iter()
              .any(|table_metadata_column| table_metadata_column.column_name == column_name) {
            return Err(
              NollaDBError::Internal(
                format!("Duplicate column name: {}", &column_name)
              )
            );
          }

          let column_datatype = match &column.data_type {
            DataType::SmallInt(_) => "Integer", // bytes
            DataType::Int(_) => "Integer", // bytes
            DataType::BigInt(_) => "Integer", // bytes
            DataType::Text => "Text",
            DataType::Varchar(_) => "Text", // bytes
            DataType::Boolean => "Bool",
            DataType::Real => "Real",
            DataType::Float(_) => "Real", // precision
            DataType::Double => "Real",
            DataType::Decimal(_, _) => "Real", // precision
            _ => {
              eprintln!("not matched on custom type");
              "Invalid"
            }
          };

          let mut is_primary_key: bool = false;
          let mut is_unique_constraint: bool = false;
          let mut is_not_null_constraint: bool = false;

          for column_option in &column.options {
            match column_option.option {
              ColumnOption::Unique {
                is_primary
              } => {
                // 只有 Integer 和 Text 类型可以作为 PRIMARY KEY 和 Unique 约束
                if column_datatype == "Bool" ||
                   column_datatype == "Real" ||
                   !is_primary { continue; }

                // 这里还要检查创建表时，表里面是否已经有 PRIMARY KEY
                if table_metadata_columns
                    .iter()
                    .any(|table_metadata_column| table_metadata_column.is_primary_key == true) {
                  return Err(
                    NollaDBError::Internal(
                      format!("Table '{}' has more than one PRIMARY KEY", &name)
                    )
                  );
                }

                is_primary_key = is_primary;
                is_unique_constraint = true;
                // 而只有是 PRIMARY KEY 的情况下，才可以是 NOT NULL 约束
                is_not_null_constraint = true;

              },
              ColumnOption::NotNull => {
                is_not_null_constraint = true;
              },
              _ => (),
            };
          }

          // 组装 table_metadata_columns
          table_metadata_columns.push(SchemaOfSQLColumn {
            column_name,
            column_datatype: column_datatype.to_string(),
            is_primary_key,
            is_unique_constraint,
            is_not_null_constraint,
          });
        }

        // TODO: 处理 constraints
        for constraint in constraints {
          println!("{:?}", constraint);
        }

      },
      _ => return Err(NollaDBError::Internal("Parsing CREATE SQL query error".to_string())),
    }

    match option_table_name {
      Some(table_name) => Ok(CreateQuery {
          table_name,
          table_metadata_columns,
        }),
      _ => return Err(NollaDBError::Internal("Parsing CREATE SQL query error".to_string())),
    }
  }
}
