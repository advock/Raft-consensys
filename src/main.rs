#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use once_cell::sync::OnceCell;
use rocket::serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;

#[macro_use]
extern crate rocket_contrib;

mod Core;
mod LeaderElection;
mod shared;
use std::collections::HashMap;
use std::{thread, usize};
use LeaderElection::Node;

#[derive(serde::Deserialize)]
struct AppendEntriesRequest {
    data: u32,
    done: bool,
}

#[derive(Debug, PartialEq)]
struct AppendEntriesResponse {
    term: u32,
    success: bool,
}

#[get("/startnodes/<value>")]
fn startnodes(value: Option<u32>) {
    if value == None {
        LeaderElection::RunNode(None);
    } else {
        LeaderElection::RunNode(value);
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/raft", routes![startnodes, appendEntries])
}

#[get("/appendEntries/<value>")]
fn appendEntries(value: Option<u32>) -> &'static str {
    let (tx, rx) = shared::create_channel();
    startnodes(value);
    let received = rx.recv().unwrap();

    let my_str = format!("value {} added to the log", received);

    static STATIC_STRING: OnceCell<String> = OnceCell::new();
    let string_ref = STATIC_STRING.get_or_init(|| my_str);
    &string_ref
}
