use crate::schema::users;
use diesel::AsChangeset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, AsChangeset)]
#[table_name = "users"]
pub struct UserRequest {
  pub name: String,
  pub level: Option<i32>,
  pub email: Option<String>,
  pub last_submitted: Option<String>,
}
