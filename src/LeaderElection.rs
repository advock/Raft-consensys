use rand::Rng;
//use rocket::http::{ContentType, Header, HeaderMap};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::task::JoinHandle;
use serde::ser::Serializer;
use serde::Deserializer;
use serde_json;
//use serde_json::to_string;
use std::collections::HashMap;
use std::fmt::Debug;
//use std::fmt::Display;
use crate::shared;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::appendEntries;
//use std::time::{SystemTime, UNIX_EPOCH};

//const CONTENT_TYPE: HeaderName = HeaderName::from_static("Content-Type");

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum NodeState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: usize,
    state: NodeState,
    #[serde(
        serialize_with = "serialize_instant",
        deserialize_with = "deserialize_instant"
    )]
    timer: Instant,
    votes_received: usize,
    log: Vec<u32>,
    leader: Option<usize>,
    term: u32,
}

pub fn RunNode(value: Option<u32>) {
    let num_nodes = 5;
    println!("{} Nodes Created", num_nodes);
    let nodes = create_nodes(num_nodes);
    let nodes_arc = Arc::new(Mutex::new(nodes));
    let mut handles = Vec::new();
    let tx = shared::get_sender();
    let rx = shared::get_receiver();

    for id in 0..num_nodes {
        println!("{}", id);
        let nodes_arc_clone = nodes_arc.clone();
        let value_clone = value.clone();
        let tx_clone = tx.clone();
        // let (mut tx, mut rx): (Sender<Node>, Receiver<Node>) = channel();
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut node = nodes_arc_clone.lock().unwrap()[id].clone();
            println!("{}", id);
            println!("");

            loop {
                match node.state {
                    NodeState::Follower => {
                        let elapsed = node.timer.elapsed().as_millis();

                        let start: u128 = 0;
                        let end = start + 1500;

                        println!("{:?}", node);
                        println!("");
                        if elapsed >= rng.gen_range(start..=end) {
                            println!("node {} has turned into cadidate", node.id);
                            println!("");
                            node.state = NodeState::Candidate;
                            node.timer = Instant::now();
                            node.leader = None;
                            node.term = node.term + 1;
                            println!("Node {} became a candidate", node.id);
                            println!("");
                            broadcast_request_vote(&mut node, &nodes_arc_clone);
                        }
                    }
                    NodeState::Candidate => {
                        let elapsed = node.timer.elapsed().as_millis();
                        let start: u128 = 0;
                        let end = start + 1500;
                        println!("{:?}", node);
                        println!("");
                        if elapsed >= rng.gen_range(start..=end) && node.votes_received >= 3 {
                            node.timer = Instant::now();
                            node.state = NodeState::Leader;
                            node.leader = None;
                            node.term = node.term + 1;
                            println!("Node {} started a new election", node.id);
                            println!("");
                            broadcast_request_vote(&mut node, &nodes_arc_clone);

                            // changes to change in other nodes
                            let mut nodes = nodes_arc_clone.lock().unwrap();
                            for other_node in nodes.iter_mut() {
                                if other_node.id != node.id {
                                    other_node.state = NodeState::Candidate;
                                    other_node.timer = Instant::now();
                                    other_node.leader = Some(node.id);
                                    other_node.term = node.term;
                                }
                            }
                            println!("changes regarding Leader in other nodes done");
                        }
                    }
                    NodeState::Leader => {
                        println!("Node {} is the leader", node.id);
                        let _elapsed = node.timer.elapsed().as_millis();
                        let mut nodes_map = HashMap::new();
                        let nodes_mutex = nodes_arc_clone.lock().unwrap();
                        for node in nodes_mutex.iter() {
                            nodes_map.insert(node.id, node.clone());
                        }
                        if value != None {
                            let appende = &node.append_entries(&[value_clone.unwrap()]);
                            broadcast_heartbeat(&node, &nodes_map);
                            println!("Broadcast completed");
                            print!("append status  {:?}", !appende);
                            if !*appende {
                                println!("log added");
                                let final_log = node.log.clone();

                                tx_clone.send(value_clone.unwrap()).unwrap();
                                println!("log send to main file");
                            }
                            break;
                        }

                        thread::sleep(Duration::from_millis(500));
                    }
                }

                let mut nodes = nodes_arc_clone.lock().unwrap();
                nodes[node.id] = node.clone();
                drop(nodes);

                thread::sleep(Duration::from_millis(1150));
                node = nodes_arc_clone.lock().unwrap()[id].clone();
            }
        });

        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn create_nodes(num_nodes: usize) -> Vec<Node> {
    let mut nodes = Vec::new();

    for id in 0..num_nodes {
        nodes.push(Node {
            id,
            state: NodeState::Follower,
            timer: Instant::now(),
            votes_received: 0,
            log: vec![0],
            leader: None,
            term: 0,
        });
    }

    nodes
}

fn broadcast_request_vote(node: &mut Node, nodes_arc: &Arc<Mutex<Vec<Node>>>) {
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
                node.votes_received += 1;
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

fn serialize_instant<S>(time: &Instant, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let timestamp = time.elapsed().as_secs_f64();
    serializer.serialize_f64(timestamp)
}

fn deserialize_instant<'de, D>(deserializer: D) -> Result<Instant, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = f64::deserialize(deserializer)?;
    let duration = Duration::from_secs_f64(timestamp);
    Ok(Instant::now() - duration)
}

#[derive(Debug, PartialEq)]
struct AppendEntriesRequest {
    term: u32,
    leader_id: usize,
    prev_log_index: u32,
    prev_log_term: u32,
    entries: Vec<u32>,
    //leader_commit: u32,
}

// struct AppendEntriesResponse {
//     term: [u32],
//     success: bool,
// }

impl Node {
    fn append_entries(&mut self, entries: &[u32]) -> bool {
        let last_entry_index = self.log.len() as u32 - 1;
        let mut success = false;
        let mut term = self.term;
        if entries.is_empty() {
            success = true;
        } else if self.log.is_empty() || self.log[last_entry_index as usize] == entries[0] {
            self.log.extend_from_slice(entries);
            success = true;
        } else {
            let entry_index = self.log.binary_search(&entries[0]).unwrap_or_else(|x| x);
            if entry_index == 0 {
                success = true;
            } else if self.log[entry_index as usize - 1] == entries[0] {
                self.log.truncate(entry_index as usize - 1);
                self.log.extend_from_slice(entries);
                success = true;
            }
        }
        success
    }
}

fn broadcast_heartbeat(node: &Node, nodes: &HashMap<usize, Node>) {
    let mut nodes_map = nodes.clone();
    let current_term = node.term;
    let leader_id = node.id;
    let prev_log_index = node.log.len() as u32 - 1;
    let prev_log_term = node.term;
    let entries = node.log.clone();

    for (id, n) in &mut nodes_map {
        if *id == leader_id {
            continue;
        }
        let request = AppendEntriesRequest {
            term: current_term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries: entries.clone(),
        };

        // send request to node using some networking library like reqwest, hyper or rocket
        // here we just print the request for demonstration purposes
        println!("Sending heartbeat to node {}: {:?}", id, request);
    }
}
