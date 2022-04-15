use url::Url;

pub mod types;

pub mod edges;

// TODO: more info, i.e. api path etc, for error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error making request request")]
    RequestError(#[from] reqwest::Error),
}

pub struct ClientConfig {
    auth_token: String,
    api_url: Option<Url>,
}

pub struct Client {
    conf: ClientConfig,
    c: reqwest::Client,
}

impl Client {
    pub fn new(conf: ClientConfig) -> Self {
        Client {
            c: reqwest::Client::new(),
            conf,
        }
    }

    pub(crate) async fn make_request<T, R>(&self, path: &str, method: reqwest::Method, body: Option<T>) -> Result<R, Error>
        where T: serde::Serialize,
              R: serde::de::DeserializeOwned
    {
        let api_url = self.conf.api_url.clone().unwrap_or_else(|| {
            "https://api.ngrok.com".parse::<Url>().unwrap()
        });

        let mut builder = self.c.request(method, api_url.join(path).unwrap())
            .bearer_auth(&self.conf.auth_token);
        builder = match body {
            Some(b) => builder.json(&b),
            None => builder,
        };

        let resp = builder.send().await?;
        resp.json().await
            .map_err(|e| e.into())
    }
}
