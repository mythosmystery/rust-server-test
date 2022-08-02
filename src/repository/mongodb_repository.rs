use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::{UpdateUser, User};
use mongodb::{
  bson::{doc, extjson::de::Error, oid::ObjectId}, //modify here
  results::{DeleteResult, InsertOneResult, UpdateResult},
  sync::{Client, Collection},
};

pub struct MongoRepo {
  col: Collection<User>,
}

impl MongoRepo {
  pub fn init() -> Self {
    dotenv().ok();
    let uri = match env::var("MONGO_URI") {
      Ok(v) => v.to_string(),
      Err(_) => format!("Error loading env variable"),
    };
    let client = Client::with_uri_str(uri).unwrap();
    let db = client.database("typenotes");
    let col: Collection<User> = db.collection("User");
    MongoRepo { col }
  }

  pub fn create_user(&self, user: User) -> Result<InsertOneResult, String> {
    let hash = bcrypt::hash(&user.password, 10).unwrap();
    let data = User {
      id: None,
      name: user.name.to_owned(),
      email: user.email.to_owned(),
      password: hash.to_owned(),
      created_at: user.created_at,
    };
    let user_detail = self.col.insert_one(data, None);
    match user_detail {
      Ok(user) => Ok(user),
      Err(_) => Err(String::from("Error creating user")),
    }
  }

  pub fn get_user(&self, id: &String) -> Result<User, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let user_detail = self
      .col
      .find_one(filter, None)
      .ok()
      .expect("Error getting user's detail");
    Ok(user_detail.unwrap())
  }

  pub fn update_user(&self, id: &String, new_user: UpdateUser) -> Result<UpdateResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let new_doc = doc! {
        "$set":
            {
                "id": id,
                "name": new_user.name,
                "email": new_user.email,
            },
    };
    let updated_doc = self
      .col
      .update_one(filter, new_doc, None)
      .ok()
      .expect("Error updating user");
    Ok(updated_doc)
  }

  pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let user_detail = self
      .col
      .delete_one(filter, None)
      .ok()
      .expect("Error deleting user");
    Ok(user_detail)
  }

  pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
    let cursors = self
      .col
      .find(None, None)
      .ok()
      .expect("Error getting list of users");
    let users = cursors.map(|doc| doc.unwrap()).collect();
    Ok(users)
  }
}
