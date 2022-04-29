use futures::stream::StreamExt;

use ngrok_api::types::HTTPSEdge;
use ngrok_api::{Client, ClientConfig, Error};

#[tokio::main]
async fn main() {
    let token = std::env::var("NGROK_API_KEY").expect("Set NGROK_API_KEY env var");

    let c = Client::new(ClientConfig {
        auth_token: token.to_owned(),
        api_url: None,
    });

    let resp = c
        .edges()
        .https()
        .create(&ngrok_api::types::HTTPSEdgeCreate {
            description: "made from rust".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    println!("{:?}", resp);

    // list all edges in the account
    let mut stream = c.edges().https().list(Default::default()).pages().await;
    while let Some(page) = stream.next().await {
        println!("page: {:?}", page);
    }

    // vec of edges
    let edges: Result<Vec<HTTPSEdge>, Error> = c
        .edges()
        .https()
        .list(Default::default())
        .https_edges()
        .await
        .collect::<Vec<Result<HTTPSEdge, Error>>>()
        .await
        .into_iter()
        .collect();
    println!("edges: {:?}", edges);
}
