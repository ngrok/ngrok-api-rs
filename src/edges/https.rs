use crate::types;
use crate::Error;

pub struct Client<'a> {
    c: &'a crate::Client,
}

impl<'a> Client<'a> {
    pub fn new(c: &'a crate::Client) -> Self {
        Self { c }
    }

    pub async fn create(&self, req: types::HTTPSEdgeCreate) -> Result<types::HTTPSEdge, Error> {
        self.c
            .make_request("/edges/https", reqwest::Method::POST, None, Some(req))
            .await
    }

    pub async fn list_unpaged(&self, paging: types::Paging) -> Result<types::HTTPSEdgeList, Error> {
        self.c
            .make_request("/edges/https", reqwest::Method::GET, Some(paging), None::<Option<()>>)
            .await
    }
}
