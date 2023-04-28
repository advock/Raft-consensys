use rand::Rng;
use rocket::http::{ContentType, Header, HeaderMap};
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};

//const CONTENT_TYPE: HeaderName = HeaderName::from_static("Content-Type");

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Clone)]
struct Node {
    id: usize,
    state: NodeState,
    timer: Instant,
    votes_received: usize,
    leader: Option<usize>,
    term: u32,
}

pub fn RunNode() {
    let num_nodes = 5;
    let nodes = create_nodes(num_nodes);

    let nodes_arc = Arc::new(Mutex::new(nodes));

    let mut handles = Vec::new();

    for id in 0..num_nodes {
        let nodes_arc_clone = nodes_arc.clone();

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut node = nodes_arc_clone.lock().unwrap()[id].clone();

            loop {
                match node.state {
                    NodeState::Follower => {
                        let elapsed = node.timer.elapsed().as_millis();
                        let start: u128 = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis();
                        let end = start + 1500;
                        if elapsed >= rng.gen_range(start..=end) {
                            node.state = NodeState::Candidate;
                            node.timer = Instant::now();
                            node.votes_received = 1;
                            node.leader = None;
                            println!("Node {} became a candidate", node.id);
                            broadcast_request_vote(&node, &nodes_arc_clone);
                        }
                    }
                    NodeState::Candidate => {
                        let elapsed = node.timer.elapsed().as_millis();
                        let start: u128 = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis();
                        let end = start + 1500;
                        if elapsed >= rng.gen_range(start..=end) {
                            node.timer = Instant::now();
                            node.votes_received = 1;
                            node.leader = None;
                            println!("Node {} started a new election", node.id);
                            broadcast_request_vote(&node, &nodes_arc_clone);
                        }
                    }
                    NodeState::Leader => {
                        println!("Node {} is the leader", node.id);
                        let mut nodes_map = HashMap::new();
                        let nodes_mutex = nodes_arc_clone.lock().unwrap();
                        for node in nodes_mutex.iter() {
                            nodes_map.insert(node.id, node.clone());
                        }

                        // broadcast_heartbeat(&node, &nodes_map);
                        thread::sleep(Duration::from_millis(500));
                    }
                }

                let mut nodes = nodes_arc_clone.lock().unwrap();
                nodes[node.id] = node.clone();
                drop(nodes);

                thread::sleep(Duration::from_millis(100));
                node = nodes_arc_clone.lock().unwrap()[id].clone();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn create_nodes(num_nodes: usize) -> Vec<Node> {
    let mut nodes = Vec::new();

    for id in 0..num_nodes {
        nodes.push(Node {
            id,
            state: NodeState::Follower,
            timer: Instant::now(),
            votes_received: 0,
            leader: None,
            term: 0,
        });
    }

    nodes
}

fn broadcast_request_vote(node: &Node, nodes_arc: &Arc<Mutex<Vec<Node>>>) {
    let mut nodes = nodes_arc.lock().unwrap();

    for i in 0..nodes.len() {
        if i == node.id {
            continue;
        }

        let mut other_node = nodes[i].clone();

        if other_node.state != NodeState::Leader {
            other_node.state = NodeState::Follower;
            nodes[i] = other_node.clone();

            let mut rng = rand::thread_rng();

            let vote_granted = rng.gen_bool(0.5);

            if vote_granted {
                //node.votes_received += 1;
                println!(
                    "Node {} granted its vote to node {}",
                    other_node.id, node.id
                );
            } else {
                println!("Node {} denied its vote to node {}", other_node.id, node.id);
            }
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// struct HeartbeatRequestBody {
//     leader_id: usize,
//     term: u32,
// }
// #[derive(Serialize, Deserialize, Debug)]
// struct HeartbeatResponseBody {
//     success: bool,
//     term: u32,
// }

// fn broadcast_heartbeat(node: &Node, nodes: &HashMap<usize, Node>) {
//     for (_, follower_node) in nodes.iter() {
//         if follower_node.id != node.id {
//             let client = reqwest::blocking::Client::new();
//             let request_body = HeartbeatRequestBody {
//                 leader_id: node.id,
//                 term: node.term,
//             };
//             let url = format!("http://{}/heartbeat", follower_node.id);
//             let response = client
//                 .post(url)
//                 .header(CONTENT_TYPE,ContentType::JSON)
//                 .json(&request_body)
//                 .send();
//             match response {
//                 Ok(response) => {
//                     let response_body = response.json::<HeartbeatResponseBody>().unwrap();
//                     if response_body.term > node.term {
//                         node.term = response_body.term;
//                         //node.vote_granted = false;
//                     }
//                 }
//                 Err(e) => eprintln!(
//                     "Error sending heartbeat to node {}: {}",
//                     follower_node.id, e
//                 ),
//             }
//         }
//     }
// }

// fn broadcast_hearetbeat(nodes_arc: Arc<Mutex<Vec<Node>>>, id: usize) {
//     let nodes_arc_clone = nodes_arc.clone();
//     let node = nodes_arc_clone.lock().unwrap()[id].clone();

//     if node.state != NodeState::Leader {
//         return;
//     }

//     let heartbeat_request_body = HeartbeatRequestBody {
//         term: node.term,
//         leader_id: node.id,
//        // leader_commit: node.commit_index,
//     };

//     for (i, n) in nodes_arc.lock().unwrap().iter().enumerate() {
//         if n.id == id {
//             continue;
//         }

//         let url = &format!("http://localhost:{}", n.id + 8000);

//         let client = reqwest::blocking::Client::new();

//         let response = client
//             .post(url)
//             .header(Header::new(CONTENT_TYPE, ContentType::JSON.to_string()))
//             .body(json(&heartbeat_request_body).to_string())
//             .send();

//         if let Ok(resp) = response {
//             if let Ok(body) = resp.json::<HeartbeatResponseBody>() {
//                 let mut nodes = nodes_arc_clone.lock().unwrap();
//                 let mut current_node = &mut nodes[id];

//                 if body.term > current_node.current_term {
//                     current_node.term = body.term;
//                     current_node.state = NodeState::Follower;
//                    // current_node.voted_for = None;
//                     current_node.leader = None;
//                 }
//             }
//         }
//     }
// }
