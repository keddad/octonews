#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize, Debug)]
struct News {
    title: String,
    text: String,
    url: String,
    posted: String,
    links: Vec<String>,
}

#[get("/probe")]
fn index() -> Status {
    Status::Ok
}

#[post("/submit", data = "<n>")]
async fn submit(n: Json<News>) -> Status {
    print!("{:#?}", n);
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit])
}
