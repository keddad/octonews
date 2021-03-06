use log::debug;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes, State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
struct News {
    title: String,
    text: String,
    uri: String,
    posted: String,
    links: Vec<String>,
}

#[get("/probe")]
async fn index() -> Status {
    Status::Ok
}

#[post("/submit", data = "<n>")]
async fn submit(n: Json<News>, red: &State<redis::Client>) -> Status {
    if n.title.len() == 0 || n.uri.len() == 0 {
        return Status::BadRequest;
    }

    let mut con = red
        .get_async_connection()
        .await
        .expect("Failed to get_async_connection");

    let _: () = redis::cmd("XADD")
        .arg(&[
            "deduplicator",
            "*",
            "news",
            &serde_json::to_string(&n.0).unwrap(),
        ])
        .query_async(&mut con)
        .await
        .expect("Failed to put object to stream");

    debug!("Put {} to deduplicator", n.uri);
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    rocket::build()
        .manage(redis::Client::open("redis://redis/").expect("Failed to connect to Redis"))
        .mount("/", routes![index, submit])
}
