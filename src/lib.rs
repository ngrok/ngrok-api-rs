use url::Url;

mod clients;
mod errors;
pub mod types;

pub use clients::*;
pub use errors::*;

/// `ClientBuilder` is a builder for [Client]
pub struct ClientBuilder {
    // required options
    api_key: String,

    // optional
    api_url: Option<Url>,
    reqwest_client: Option<reqwest::Client>,
}

/// `ClientBuilder` allows customizing various options for the [Client]
impl ClientBuilder {
    /// Construct a builder to customize and then build the client.
    pub fn new(api_key: String) -> ClientBuilder {
        ClientBuilder {
            api_key,
            api_url: None,
            reqwest_client: None,
        }
    }

    /// Set an API url to connect to. By default, the public ngrok API will be used.
    pub fn api_url(&mut self, api_url: Url) -> &mut Self {
        self.api_url = Some(api_url);
        self
    }

    /// Set a custom reqwest client to use for http requests to the ngrok API.
    pub fn reqwest_client(&mut self, client: reqwest::Client) -> &mut Self {
        self.reqwest_client = Some(client);
        self
    }

    /// Build the client, applying any options set.
    pub fn build(self) -> Client {
        Client {
            api_key: self.api_key,
            api_url: self
                .api_url
                .unwrap_or_else(|| Url::parse("https://api.ngrok.com").unwrap()),
            c: self.reqwest_client.unwrap_or_else(reqwest::Client::new),
        }
    }
}

/// An ngrok API client.
#[derive(Clone, Debug)]
pub struct Client {
    /// The ngrok API Key to authenticate with. See the [API documentat](https://ngrok.com/docs/api) for more information on creating an ngrok API key.
    api_key: String,
    /// The API URL base, such as `"https://api.ngrok.com"`.
    api_url: Url,
    c: reqwest::Client,
}

impl Client {
    /// Create a new API client
    ///
    /// # Examples
    ///
    /// ```
    /// use ngrok_api::Client;
    ///
    /// let c = Client::new("EXAMPLETOKEN123456789012345_EXAMPLESECRETIIIIIIIV".to_string());
    /// // use methods like 'c.reserved_domains().list(...).await' to make API calls.
    /// ```
    pub fn new(api_key: String) -> Self {
        ClientBuilder::new(api_key).build()
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
        let api_url = &self.api_url;

        let mut builder = self
            .c
            .request(method.clone(), api_url.join(path).unwrap())
            .bearer_auth(&self.api_key)
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

    pub(crate) async fn get_by_uri<R>(&self, uri: &str) -> Result<R, Error>
    where
        R: serde::de::DeserializeOwned,
    {
        let builder = self
            .c
            .request(reqwest::Method::GET, uri)
            .bearer_auth(&self.api_key)
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
