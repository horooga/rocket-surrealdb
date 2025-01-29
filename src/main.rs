mod error;
mod routes;

async fn init() -> Result<(), surrealdb::Error> {
    DB.connect::<Ws>("localhost:8001").await?;

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
pub async fn rocket() -> _ {
    std::env::set_var("ROCKET_PORT", "8000");
    init().await.expect("Something went wrong, shutting down");
    rocket::build().mount(
        "/",
        routes![
            routes::create_message,
            routes::list_messages,
        ],
    )
}
