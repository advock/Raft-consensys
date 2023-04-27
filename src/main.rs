#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};

struct RequestBody {
    term: u32,
    candidateId: u32,
    lastLogIndex: i32,
    lastLogTerm: u32,
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    term: u32,
    voteGranted: bool,
}

#[post("/vote", format = "json", data = "<request_body>")]
fn vote(
    request: rocket::serde::json::Json<RequestBody>,
) -> rocket::serde::json::Json<ResponseBody> {
    // Dummy response
    let response = Response {
        term: request.term,
        voteGranted: true,
    };
    rocket::serde::json::Json(response_body)
}

fn main() {
    rocket::ignite().mount("/", routes![vote]).launch();
}
