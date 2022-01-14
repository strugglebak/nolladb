mod datatype;
mod index;
mod row;

use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use row::Row;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Table {
  pub primary_key: String,
  pub table_name: String,
  pub indexes: HashMapM<String, String>,
  pub most_recent_row_id: i64,
  pub rows: Rc<RefCell<HashMap<String, Row>>>,
}
