use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug)]
struct Node {
    id: usize,
    state: NodeState,
    timer: Instant,
    votes_received: usize,
    leader: Option<usize>,
}

pub fn main() {
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
                        if elapsed >= rng.gen_range(1500, 3001) {
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
                        if elapsed >= rng.gen_range(1500, 3001) {
                            node.timer = Instant::now();
                            node.votes_received = 1;
                            node.leader = None;
                            println!("Node {} started a new election", node.id);
                            broadcast_request_vote(&node, &nodes_arc_clone);
                        }
                    }
                    NodeState::Leader => {
                        println!("Node {} is the leader", node.id);
                        broadcast_heartbeat(&node, &nodes_arc_clone);
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
