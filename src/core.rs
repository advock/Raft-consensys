struct raftNode {
    current_term: u64,
    voted_for: Option<u64>,
    log: Vec<LogEntry>,
}

impl RaftNode {
    fn append_entries(&mut self, req: AppendEntriesRequest) -> AppendEntriesResponse {
        let mut response = AppendEntriesResponse {
            term: self.current_term,
            success: false,
        };

        // If leader's term is less than ours, reject
        if req.term < self.current_term {
            return response;
        }

        self.current_term = req.term;
        self.voted_for = None;

        let prev_log_index = req.prev_log_index as usize;
        let prev_log_term = req.prev_log_term;

        // If our log doesn't contain the previous entry, reject
        if prev_log_index >= self.log.len() || self.log[prev_log_index].term != prev_log_term {
            return response;
        }

        // Delete conflicting entries
        let mut i = prev_log_index + 1;
        let mut j = 0;
        while i < self.log.len() && j < req.entries.len() {
            if self.log[i].term != req.entries[j].term {
                self.log.truncate(i);
                break;
            }
            i += 1;
            j += 1;
        }

        // Append new entries
        self.log.extend_from_slice(&req.entries[j..]);

        // Update commit index
        if req.leader_commit > self.commit_index {
            self.commit_index = std::cmp::min(req.leader_commit, self.log.len() as u64 - 1);
        }

        response.success = true;
        response
    }
}
