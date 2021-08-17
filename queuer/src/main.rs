#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize, Debug)]
struct News<'a> {
    title: &'a str,
    text: &'a str,
    url: &'a str,
    posted: &'a str,
    links: Vec<&'a str>,
}

#[get("/probe")]
fn index() -> Status {
    Status::Ok
}

#[get("/submit", data = "<n>")]
async fn submit(n: Json<News<'_>>) -> Status {
    print!("{:#?}", n);
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit])
}
