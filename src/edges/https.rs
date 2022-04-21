use crate::types;
use crate::Error;

use futures::stream::{Stream, TryStreamExt};

#[derive(Clone)]
pub struct Client {
    pub(crate) c: crate::Client,
}

impl Client {
    pub fn new(c: crate::Client) -> Self {
        Self { c }
    }

    pub async fn create(&self, req: types::HTTPSEdgeCreate) -> Result<types::HTTPSEdge, Error> {
        self.c
            .make_request("/edges/https", reqwest::Method::POST, None, Some(req))
            .await
    }

    // Use list instead
    pub async fn list_page(&self, paging: types::Paging) -> Result<types::HTTPSEdgeList, Error> {
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
    pub async fn list(&self) -> HTTPSEdgeListResp {
        HTTPSEdgeListResp::new(
            self.clone(),
            types::Paging {
                limit: None,
                before_id: None,
            },
        )
    }
}

pub struct HTTPSEdgeListResp {
    c: std::sync::Arc<Client>,
    paging: types::Paging,
}

impl HTTPSEdgeListResp {
    pub(crate) fn new(c: Client, paging: types::Paging) -> Self {
        Self {
            c: std::sync::Arc::new(c),
            paging,
        }
    }

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
            if s.init {
                let page = match s.c.list_page(s.paging.clone()).await {
                    Err(e) => return Some((Err(e), s)),
                    Ok(p) => p,
                };
                let next = page.next_page_uri.clone();
                return Some((
                    Ok(page),
                    State {
                        c: s.c.clone(),
                        paging: s.paging,
                        init: false,
                        cur_uri: next,
                    },
                ));
            }
            match s.cur_uri {
                None => None,
                Some(ref uri) => {
                    let page: types::HTTPSEdgeList = match s.c.c.get_by_uri(uri).await {
                        Err(e) => return Some((Err(e), s)),
                        Ok(p) => p,
                    };
                    let next = page.next_page_uri.clone();
                    Some((
                        Ok(page),
                        State {
                            c: s.c.clone(),
                            paging: s.paging,
                            init: false,
                            cur_uri: next,
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
