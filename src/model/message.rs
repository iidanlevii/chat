use futures::stream::TryStreamExt;
use std::env;

use mongodb::{
    bson::{doc, DateTime},
    options::FindOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};

use crate::{
    ctx::Ctx,
    error::{Error, Result},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub from: i64,
    pub to: i64,
    pub message: String,
    pub stamp: DateTime,
}

#[derive(Debug, Deserialize)]
pub struct MessageToCreate {
    to: i64,
    message: String,
}

#[derive(Debug, Clone)]
pub struct MessageModel {
    collection: Collection<Message>,
}

impl MessageModel {
    pub async fn new() -> Result<Self> {
        let uri = env::var("MONGO_URL").expect("no mongo env");

        let client = Client::with_uri_str(uri)
            .await
            .map_err(|_| Error::MongoConnectionError)?;

        let db = client.database("chat");
        let collection = db.collection::<Message>("messages");

        // TODO: indexes
        Ok(Self { collection })
    }
}

impl MessageModel {
    pub async fn add_message(&self, ctx: Ctx, message: MessageToCreate) -> Result<()> {
        self.collection
            .insert_one(
                Message {
                    from: ctx.user_id(),
                    to: message.to,
                    message: message.message,
                    stamp: DateTime::now(),
                },
                None,
            )
            .await
            .map_err(|_| Error::MongoInsertError)?;

        Ok(())
    }

    pub async fn find_messages_by_user(&self, user_id: i64) -> Result<Vec<Message>> {
        let find_options = FindOptions::builder().sort(doc! { "stamp": 1 }).build();

        let cursor = self
            .collection
            .find(
                doc! { "$or": [ { "from": user_id }, { "to": user_id } ] },
                find_options,
            )
            .await
            .map_err(|_| Error::MongoFindError)?;

        let messages: Vec<Message> = cursor
            .try_collect()
            .await
            .map_err(|_| Error::MongoFindError)?;

        for message in messages.iter() {
            println!("{:?}", message);
        }

        Ok(messages)
    }
}
