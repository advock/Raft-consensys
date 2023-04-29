#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket_contrib;

mod Core;
mod LeaderElection;

#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
struct RequestBodyEntry {
    term: u32,
    command: String,
}
#[derive(serde::Deserialize)]
struct AppendEntriesRequest {
    term: u32,
    leader_id: u32,
    prev_log_index: i32,
    prev_log_term: u32,
    entries: Vec<RequestBodyEntry>,
    leader_commit: u64,
}

struct AppendEntriesResponse {
    term: u64,
    success: bool,
}

#[get("/")]
fn startnodes() -> &'static str {
    //rocket::serde::json::Json<ResponseBody>
    // Dummy response
    let out = LeaderElection::RunNode();
    let boxed_str = out.into_boxed_str();
    let static_str = Box::leak(boxed_str);
    static_str
}

// #[post("/appendEntries", format = "json", data = "<request>")]
// fn appendEntries(
//     request: rocket::serde::json::Json<RequestBody>,
// ) -> rocket::serde::json::Json<ResponseBody> {
//     let response = Response {
//         term: request.term,
//         voteGranted: true,
//     };
//     rocket::serde::json::Json(request_body)
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![startnodes])
}

// fn broadcast_heartbeat(node: &Node, nodes: &HashMap<u32, Node>) {
//     for (_, follower_node) in nodes.iter() {
//         if follower_node.id != node.id {
//             let client = reqwest::blocking::Client::new();
//             let request_body = HeartbeatRequestBody {
//                 leader_id: node.id,
//                 term: node.term,
//             };
//             let url = format!("http://{}/heartbeat", follower_node.address);
//             let response = client
//                 .post(url)
//                 .header(ContentType::JSON)
//                 .json(&request_body)
//                 .send();
//             match response {
//                 Ok(response) => {
//                     let response_body = response.json::<HeartbeatResponseBody>().unwrap();
//                     if response_body.term > node.term {
//                         node.term = response_body.term;
//                         node.vote_granted = false;
//                     }
//                 }
//                 Err(e) => eprintln!(
//                     "Error sending heartbeat to node {}: {}",
//                     follower_node.id, e
//                 ),
//             }
//         }
//     }
