// mod main;
// struct raftNode {
//     current_term: u64,
//     voted_for: Option<u64>,
//     log: Vec<LogEntry>,
// }

// impl RaftNode {
//     fn append_entries(&mut self, req: AppendEntriesRequest) -> AppendEntriesResponse {
//         let mut response = AppendEntriesResponse {
//             term: self.current_term,
//             success: false,
//         };

//         // If leader's term is less than ours, reject
//         if req.term < self.current_term {
//             return response;
//         }

//         self.current_term = req.term;
//         self.voted_for = None;

//         let prev_log_index = req.prev_log_index as usize;
//         let prev_log_term = req.prev_log_term;

//         // If our log doesn't contain the previous entry, reject
//         if prev_log_index >= self.log.len() || self.log[prev_log_index].term != prev_log_term {
//             return response;
//         }

//         // Delete conflicting entries
//         let mut i = prev_log_index + 1;
//         let mut j = 0;
//         while i < self.log.len() && j < req.entries.len() {
//             if self.log[i].term != req.entries[j].term {
//                 self.log.truncate(i);
//                 break;
//             }
//             i += 1;
//             j += 1;
//         }

//         // Append new entries
//         self.log.extend_from_slice(&req.entries[j..]);

//         // Update commit index
//         if req.leader_commit > self.commit_index {
//             self.commit_index = std::cmp::min(req.leader_commit, self.log.len() as u64 - 1);
//         }

//         response.success = true;
//         response
//     }
// }

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
