mod data_type;
mod row;
mod column;

use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use row::Row;
use column::Column;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Table {
  pub primary_key: String,
  pub table_name: String,
  pub indexes: HashMap<String, String>,
  pub most_recent_row_id: i64,
  pub rows: Rc<RefCell<HashMap<String, Row>>>,
  pub columns: Vec<Column>,
}
