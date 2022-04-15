use async_trait::async_trait;

use crate::types;
use crate::Error;
use crate::Client as BaseClient;

#[async_trait]
trait Client {
    async fn create(&self, req: types::HTTPSEdgeCreate) -> Result<types::HTTPSEdge, Error>;
}

#[async_trait]
impl Client for BaseClient {
    async fn create(&self, req: types::HTTPSEdgeCreate) -> Result<types::HTTPSEdge, Error> {
        self.make_request("/edges/https", reqwest::Method::POST, Some(req)).await
    }
}
