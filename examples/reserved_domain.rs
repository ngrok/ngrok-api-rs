use futures::stream::StreamExt;

use ngrok_api::types;
use ngrok_api::{Client, Error};

use rand::Rng;

#[tokio::main]
async fn main() {
    let token = std::env::var("NGROK_API_KEY").expect("Set NGROK_API_KEY env var");

    let c = Client::new(token.to_owned());

    let rd = c.reserved_domains();

    // random domain so the example can run without conflicting
    let rand: u64 = rand::thread_rng().gen();
    let resp = rd
        .create(&types::ReservedDomainCreate {
            name: format!("rustexample{}.ngrok.io", rand),
            ..Default::default()
        })
        .await
        .unwrap();
    println!("{:?}", resp);

    let resp = rd.get(&resp.id).await.unwrap();
    println!("{:?}", resp);

    let rds: Result<Vec<types::ReservedDomain>, Error> = rd
        .list(Default::default())
        .reserved_domains()
        .await
        .collect::<Vec<Result<_, Error>>>()
        .await
        .into_iter()
        .collect();
    println!("rds: {:?}", rds);

    // update
    let updated = rd
        .update(&types::ReservedDomainUpdate {
            id: resp.id.clone(),
            description: Some("new description".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    println!("updated: {:?}", updated);

    // delete
    rd.delete(&resp.id).await.unwrap();
    println!("deleted it");
}
