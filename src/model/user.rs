use std::env;

use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UserModel {
    collection: Collection<User>,
}

impl UserModel {
    pub async fn new() -> Result<Self> {
        let uri = env::var("MONGO_URL").expect("no mongo env");

        let client = Client::with_uri_str(uri)
            .await
            .map_err(|_| Error::MongoConnectionError)?;

        let db = client.database("users");
        let collection = db.collection::<User>("users");

        Ok(Self { collection })
    }
}

impl UserModel {
    pub async fn add_user(&self, user: User) -> Result<()> {
        let res = self
            .collection
            .find_one(doc! {"username": &user.username}, None)
            .await
            .map_err(|_| Error::MongoFindError)?;

        if res.is_some() {
            return Err(Error::MongoUserExists);
        }

        self.collection
            .insert_one(user, None)
            .await
            .map_err(|_| Error::MongoInsertError)?;

        Ok(())
    }

    pub async fn get_user(&self, user: User) -> Result<User> {
        let user = self
            .collection
            .find_one(doc! {"username": &user.username}, None)
            .await
            .map_err(|_| Error::MongoFindError)?;

        match user {
            Some(user) => Ok(user),
            None => Err(Error::MongoFindError),
        }
    }
}
