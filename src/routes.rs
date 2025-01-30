use surrealdb::opt::auth::Record;
use crate::error::Error;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageData {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    name: String,
}

#[post("/message/<id>", data = "<message>")]
pub async fn create_message(
    id: String,
    message: Json<MessageData>,
) -> Result<Json<Option<Message>>, Error> {
    let message = DB
        .create(("message", &*id))
        .content(message.into_inner())
        .await?;
    Ok(Json(message))
}

#[get("/messages")]
pub async fn list_messages() -> Result<Json<Vec<Message>>, Error> {
    let message = DB.select("message").await?;
    Ok(Json(message))
}


