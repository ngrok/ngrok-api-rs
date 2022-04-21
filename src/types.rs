use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct HTTPSEdge {
    pub id: String,
    pub description: String,
    pub metadata: String,
}

#[derive(Debug, Deserialize)]
pub struct HTTPSEdgeList {
    pub https_edges: Vec<HTTPSEdge>,
    pub uri: String,

    // pagination
    pub next_page_uri: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct HTTPSEdgeCreate {
    // human-readable description of what this edge will be used for; optional, max 255
    // bytes.
    pub description: Option<String>,
    // arbitrary user-defined machine-readable data of this edge; optional, max 4096
    // bytes.
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct Paging {
    pub before_id: Option<String>,
    pub limit: Option<u64>,
}
