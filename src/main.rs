#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};

#[derive(serde::Deserialize)]
struct RequestBody {
    term: u32,
    candidateId: u32,
    lastLogIndex: i32,
    lastLogTerm: u32,
}

#[derive(serde::Deserialize)]
struct RequestEntry {
    term: u32,
    leader_id: u32,
    prev_log_index: i32,
    prev_log_term: u32,
    entries: Vec<RequestBodyEntry>,
    leader_commit: u32,
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    term: u32,
    voteGranted: bool,
}

#[post("/vote", format = "json", data = "<request_body>")]
fn vote(
    request: rocket::serde::json::Json<RequestEntry>,
) -> rocket::serde::json::Json<ResponseBody> {
    // Dummy response
    let response = Response {
        term: request.term,
        voteGranted: true,
    };
    rocket::serde::json::Json(response_body)
}

#[post("/appendEntries", format = "json", data = "<request_body>")]
fn appendEntries(
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
    rocket::ignite().mount("/", routes![appendEntries]).launch();
}
