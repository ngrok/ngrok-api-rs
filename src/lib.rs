use serde::Deserialize;
use url::Url;

mod clients;
pub mod types;

pub use clients::*;

// TODO: more info, i.e. api path etc, for error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error making request request")]
    RequestError(#[from] reqwest::Error),
    #[error("error")]
    Ngrok(NgrokError),
    #[error("unknown error returned: {0}")]
    UnknownError(String),
}

#[derive(Debug, Deserialize)]
pub struct NgrokError {
    // TODO: parse out the code, we really shouldn't be using a string for this.
    pub error_code: String,
    pub msg: String,
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub auth_token: String,
    pub api_url: Option<Url>,
}

#[derive(Clone, Debug)]
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

    pub(crate) async fn make_request<T, R>(
        &self,
        path: &str,
        method: reqwest::Method,
        req: Option<T>,
    ) -> Result<R, Error>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned + Default,
    {
        let api_url = self
            .conf
            .api_url
            .clone()
            .unwrap_or_else(|| "https://api.ngrok.com".parse::<Url>().unwrap());

        let mut builder = self
            .c
            .request(method.clone(), api_url.join(path).unwrap())
            .bearer_auth(&self.conf.auth_token)
            .header("Ngrok-Version", "2");
        if let Some(r) = req {
            // get requests use query strings instead of bodies
            builder = match method {
                reqwest::Method::GET => builder.query(&r),
                _ => builder.json(&r),
            };
        }

        let resp = builder.send().await?;

        match resp.status() {
            reqwest::StatusCode::NO_CONTENT => return Ok(Default::default()),
            s if s.is_success() => {
                return resp.json().await.map_err(|e| e.into());
            }
            _ => {}
        }

        // if we got an error status, see if it fits into an ngrok error, and then if not return it
        // Unfortunately, that means we have to buffer it so we can try both
        let resp_bytes = resp.bytes().await?;
        if let Ok(e) = serde_json::from_slice(&resp_bytes) {
            // recognized ngrok error
            return Err(Error::Ngrok(e));
        }
        // ¯\_(ツ)_/¯
        Err(Error::UnknownError(
            String::from_utf8_lossy(&resp_bytes).into(),
        ))
    }

    // XXX: code duplication for expediency
    pub(crate) async fn get_by_uri<R>(&self, uri: &str) -> Result<R, Error>
    where
        R: serde::de::DeserializeOwned,
    {
        let builder = self
            .c
            .request(reqwest::Method::GET, uri)
            .bearer_auth(&self.conf.auth_token)
            .header("Ngrok-Version", "2");

        let resp = builder.send().await?;
        if resp.status().is_success() {
            return resp.json().await.map_err(|e| e.into());
        }
        let resp_bytes = resp.bytes().await?;
        if let Ok(e) = serde_json::from_slice(&resp_bytes) {
            return Err(Error::Ngrok(e));
        }
        Err(Error::UnknownError(
            String::from_utf8_lossy(&resp_bytes).into(),
        ))
    }
}
