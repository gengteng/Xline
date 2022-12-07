use std::sync::Arc;

use parking_lot::Mutex;

use crate::rpc::ResponseHeader;

/// Generator of `ResponseHeader`
#[derive(Debug)]
pub(crate) struct HeaderGenerator {
    /// Id of the cluster
    cluster_id: u64,
    /// Id of the member
    member_id: u64,
    /// term of curp
    term: Arc<Mutex<u64>>,
    /// revision of kv store
    revision: Arc<Mutex<i64>>,
}

impl HeaderGenerator {
    /// New `HeaderGenerator`
    pub(crate) fn new(cluster_id: u64, member_id: u64) -> Self {
        Self {
            cluster_id,
            member_id,
            term: Arc::new(Mutex::new(0)),
            revision: Arc::new(Mutex::new(1)),
        }
    }

    /// Generate `ResponseHeader`
    #[allow(dead_code)] // Will be used in the future
    pub(crate) fn gen_header(&self) -> ResponseHeader {
        ResponseHeader {
            cluster_id: self.cluster_id,
            member_id: self.member_id,
            raft_term: *self.term.lock(),
            revision: *self.revision.lock(),
        }
    }

    /// Generate `ResponseHeader` without revision, user by fast path
    pub(crate) fn gen_header_without_revision(&self) -> ResponseHeader {
        ResponseHeader {
            cluster_id: self.cluster_id,
            member_id: self.member_id,
            raft_term: *self.term.lock(),
            revision: -1,
        }
    }

    /// Set term
    #[allow(dead_code)] // Will be used in the future
    pub(crate) fn set_term(&self, term: u64) {
        *self.term.lock() = term;
    }

    /// Get revision
    pub(crate) fn revision(&self) -> i64 {
        *self.revision.lock()
    }

    /// Return Arc of revision
    pub(crate) fn revision_arc(&self) -> Arc<Mutex<i64>> {
        Arc::clone(&self.revision)
    }
}