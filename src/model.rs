use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Group {
  pub id: String,
  pub module: String,
  pub link: String,
}
