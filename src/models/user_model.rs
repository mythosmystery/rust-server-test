use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub name: String,
  pub email: String,
  pub password: String,
  #[serde(rename = "createdAt")]
  pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
  pub name: String,
  pub email: String,
}
