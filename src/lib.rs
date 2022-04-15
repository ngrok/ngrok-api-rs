use url::Url;

pub mod types;

pub mod edges;

// TODO: more info, i.e. api path etc, for error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error making request request")]
    RequestError(#[from] reqwest::Error),
    #[error("TODO: {0}")]
    TodoError(String),
}

pub struct ClientConfig {
    pub auth_token: String,
    pub api_url: Option<Url>,
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

    pub fn edges(&self) -> edges::Client {
        edges::Client::new(self)
    }

    pub(crate) async fn make_request<T, R>(&self, path: &str, method: reqwest::Method, body: Option<T>) -> Result<R, Error>
        where T: serde::Serialize,
              R: serde::de::DeserializeOwned
    {
        let api_url = self.conf.api_url.clone().unwrap_or_else(|| {
            "https://api.ngrok.com".parse::<Url>().unwrap()
        });

        let mut builder = self.c.request(method, api_url.join(path).unwrap())
            .bearer_auth(&self.conf.auth_token)
            .header("Ngrok-Version", 2);
        builder = match body {
            Some(b) => builder.json(&b),
            None => builder,
        };

        let resp = builder.send().await?;

        match resp.status() {
            c if c.is_success() => {
                resp.json().await
                    .map_err(|e| e.into())
            },
            _ => {
                Err(Error::TodoError(resp.text().await?))
            }
        }

    }
}
