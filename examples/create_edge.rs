use ngrok_api_rs::{Client, ClientConfig};

#[tokio::main]
async fn main() {
    let token = std::env::var("NGROK_API_TOKEN").expect("Set NGROK_API_TOKEN env var");

    let c = Client::new(ClientConfig{
        auth_token: token.to_owned(),
        api_url: None,
    });

    let resp = c.edges().https().create(ngrok_api_rs::types::HTTPSEdgeCreate{
        description: Some("made from rust".into()),
        ..Default::default()
    }).await.unwrap();
    println!("{:?}", resp);
}
