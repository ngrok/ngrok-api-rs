use crate::types;
use crate::Error;

use futures::stream::{Stream, TryStreamExt};

#[derive(Clone)]
pub struct Client {
    c: crate::Client,
}

impl Client {
    pub fn new(c: crate::Client) -> Self {
        Self { c }
    }

    pub async fn create(&self, req: &types::HTTPSEdgeCreate) -> Result<types::HTTPSEdge, Error> {
        self.c
            .make_request("/edges/https", reqwest::Method::POST, None, Some(req))
            .await
    }

    // Use list instead
    pub async fn list_page(&self, paging: &types::Paging) -> Result<types::HTTPSEdgeList, Error> {
        self.c
            .make_request(
                "/edges/https",
                reqwest::Method::GET,
                Some(paging),
                None::<Option<()>>,
            )
            .await
    }

    // List all HTTPSEdges
    pub fn list(&self) -> HTTPSEdgeListResp {
        HTTPSEdgeListResp {
            c: std::sync::Arc::new(self.clone()),
            paging: types::Paging {
                limit: None,
                before_id: None,
            },
        }
    }
}

pub struct HTTPSEdgeListResp {
    c: std::sync::Arc<Client>,
    paging: types::Paging,
}

impl HTTPSEdgeListResp {
    pub async fn pages(self) -> impl Stream<Item = Result<types::HTTPSEdgeList, Error>> + Unpin {
        struct State {
            c: std::sync::Arc<Client>,
            paging: types::Paging,
            init: bool,
            cur_uri: Option<String>,
        }

        let s = State {
            c: self.c,
            paging: self.paging,
            init: true,
            cur_uri: None,
        };

        Box::pin(futures::stream::unfold(s, |s| async move {
            let page_resp = match (s.init, &s.cur_uri) {
                (true, _) => {
                    // initial page
                    s.c.list_page(&s.paging).await
                }
                (false, None) => {
                    // done
                    return None;
                }
                (false, Some(uri)) => {
                    // next page
                    s.c.c.get_by_uri(uri).await
                }
            };
            match page_resp {
                Err(e) => Some((Err(e), s)),
                Ok(page) => {
                    let next = page.next_page_uri.clone();
                    Some((
                        Ok(page),
                        State {
                            init: false,
                            cur_uri: next,
                            ..s
                        },
                    ))
                }
            }
        }))
    }

    pub async fn https_edges(self) -> impl Stream<Item = Result<types::HTTPSEdge, Error>> + Unpin {
        self.pages()
            .await
            .map_ok(|page| futures::stream::iter(page.https_edges.into_iter().map(Ok)))
            .try_flatten()
    }
}
