mod error;

use surrealdb::{engine::remote::ws::{Ws, Client}, opt::auth::Root, Surreal};
use std::sync::LazyLock;
use surrealdb::opt::auth::Record;
use crate::error::Error;
use rocket::{serde::json::Json, launch, Rocket, Build, get, post, routes};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

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

async fn init() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("0.0.0.0:8001").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("database").use_db("database").await?;

    DB.query(
        "    DEFINE TABLE person SCHEMALESS
        PERMISSIONS FOR 
            CREATE, SELECT WHERE $auth,
            FOR UPDATE, DELETE WHERE created_by = $auth;
    DEFINE FIELD name ON TABLE person TYPE string;
    DEFINE FIELD created_by ON TABLE person VALUE $auth READONLY;

    DEFINE INDEX unique_name ON TABLE user FIELDS name UNIQUE;
    DEFINE ACCESS account ON DATABASE TYPE RECORD
	SIGNUP ( CREATE user SET name = $name, pass = crypto::argon2::generate($pass) )
	SIGNIN ( SELECT * FROM user WHERE name = $name AND crypto::argon2::compare(pass, $pass) )
	DURATION FOR TOKEN 15m, FOR SESSION 12h
;",
    )
    .await?;
    Ok(())
}

#[launch]
pub async fn rocket() -> Rocket<Build> {
    init().await.expect("Something went wrong, shutting down");
    rocket::build().mount(
        "/",
        routes![
            create_message,
            list_messages,
        ],
    )
}
