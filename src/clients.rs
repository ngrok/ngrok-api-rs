// Code generated for API Clients. DO NOT EDIT.

use crate::Client;

/// Abuse Reports allow you to submit take-down requests for URLs hosted by
///  ngrok that violate ngrok's terms of service.
pub mod abuse_reports {
    use crate::types;
    use crate::Error;

    /// Abuse Reports allow you to submit take-down requests for URLs hosted by
    ///  ngrok that violate ngrok's terms of service.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Creates a new abuse report which will be reviewed by our system and abuse
        /// response team. This API is only available to authorized accounts. Contact
        /// abuse@ngrok.com to request access
        pub async fn create(
            &self,
            req: &types::AbuseReportCreate,
        ) -> Result<types::AbuseReport, Error> {
            self.c
                .make_request("/abuse_reports", reqwest::Method::POST, Some(req))
                .await
        }

        /// Get the detailed status of abuse report by ID.
        pub async fn get(&self, id: &str) -> Result<types::AbuseReport, Error> {
            self.c
                .make_request(
                    &format!("/abuse_reports/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }
    }
}

pub mod agent_ingresses {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::AgentIngressList], or of [types::AgentIngress] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::AgentIngressList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::AgentIngressList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::AgentIngress] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn ingresses(
            self,
        ) -> impl Stream<Item = Result<types::AgentIngress, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.ingresses.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new Agent Ingress. The ngrok agent can be configured to connect to
        /// ngrok via the new set of addresses on the returned Agent Ingress.
        pub async fn create(
            &self,
            req: &types::AgentIngressCreate,
        ) -> Result<types::AgentIngress, Error> {
            self.c
                .make_request("/agent_ingresses", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an Agent Ingress by ID
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/agent_ingresses/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get the details of an Agent Ingress by ID.
        pub async fn get(&self, id: &str) -> Result<types::AgentIngress, Error> {
            self.c
                .make_request(
                    &format!("/agent_ingresses/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::AgentIngressList, Error> {
            self.c
                .make_request("/agent_ingresses", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all Agent Ingresses owned by this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of an Agent Ingress by ID.
        pub async fn update(
            &self,
            req: &types::AgentIngressUpdate,
        ) -> Result<types::AgentIngress, Error> {
            self.c
                .make_request(
                    &format!("/agent_ingresses/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// API Keys are used to authenticate to the [ngrok
///  API](https://ngrok.com/docs/api#authentication). You may use the API itself
///  to provision and manage API Keys but you'll need to provision your first API
///  key from the [API Keys page](https://dashboard.ngrok.com/api/keys) on your
///  ngrok.com dashboard.
pub mod api_keys {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// API Keys are used to authenticate to the [ngrok
    ///  API](https://ngrok.com/docs/api#authentication). You may use the API itself
    ///  to provision and manage API Keys but you'll need to provision your first API
    ///  key from the [API Keys page](https://dashboard.ngrok.com/api/keys) on your
    ///  ngrok.com dashboard.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::APIKeyList], or of [types::APIKey] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::APIKeyList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(self) -> impl Stream<Item = Result<types::APIKeyList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::APIKey] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn keys(self) -> impl Stream<Item = Result<types::APIKey, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.keys.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new API key. The generated API key can be used to authenticate to the
        /// ngrok API.
        pub async fn create(&self, req: &types::APIKeyCreate) -> Result<types::APIKey, Error> {
            self.c
                .make_request("/api_keys", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an API key by ID
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/api_keys/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get the details of an API key by ID.
        pub async fn get(&self, id: &str) -> Result<types::APIKey, Error> {
            self.c
                .make_request(
                    &format!("/api_keys/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::APIKeyList, Error> {
            self.c
                .make_request("/api_keys", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all API keys owned by this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of an API key by ID.
        pub async fn update(&self, req: &types::APIKeyUpdate) -> Result<types::APIKey, Error> {
            self.c
                .make_request(
                    &format!("/api_keys/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

pub mod application_sessions {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::ApplicationSessionList], or of [types::ApplicationSession] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::ApplicationSessionList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::ApplicationSessionList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::ApplicationSession] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn application_sessions(
            self,
        ) -> impl Stream<Item = Result<types::ApplicationSession, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.application_sessions.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Get an application session by ID.
        pub async fn get(&self, id: &str) -> Result<types::ApplicationSession, Error> {
            self.c
                .make_request(
                    &format!("/app/sessions/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Delete an application session by ID.
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/app/sessions/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::ApplicationSessionList, Error> {
            self.c
                .make_request("/app/sessions", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all application sessions for this account.
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }
    }
}

pub mod application_users {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::ApplicationUserList], or of [types::ApplicationUser] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::ApplicationUserList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::ApplicationUserList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::ApplicationUser] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn application_users(
            self,
        ) -> impl Stream<Item = Result<types::ApplicationUser, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.application_users.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Get an application user by ID.
        pub async fn get(&self, id: &str) -> Result<types::ApplicationUser, Error> {
            self.c
                .make_request(
                    &format!("/app/users/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Delete an application user by ID.
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/app/users/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::ApplicationUserList, Error> {
            self.c
                .make_request("/app/users", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all application users for this account.
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }
    }
}

/// Tunnel Sessions represent instances of ngrok agents or SSH reverse tunnel
///  sessions that are running and connected to the ngrok service. Each tunnel
///  session can include one or more Tunnels.
pub mod tunnel_sessions {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// Tunnel Sessions represent instances of ngrok agents or SSH reverse tunnel
    ///  sessions that are running and connected to the ngrok service. Each tunnel
    ///  session can include one or more Tunnels.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::TunnelSessionList], or of [types::TunnelSession] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::TunnelSessionList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::TunnelSessionList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::TunnelSession] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn tunnel_sessions(
            self,
        ) -> impl Stream<Item = Result<types::TunnelSession, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.tunnel_sessions.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::TunnelSessionList, Error> {
            self.c
                .make_request("/tunnel_sessions", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all online tunnel sessions running on this account.
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Get the detailed status of a tunnel session by ID
        pub async fn get(&self, id: &str) -> Result<types::TunnelSession, Error> {
            self.c
                .make_request(
                    &format!("/tunnel_sessions/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Issues a command instructing the ngrok agent to restart. The agent restarts
        /// itself by calling exec() on platforms that support it. This operation is notably
        /// not supported on Windows. When an agent restarts, it reconnects with a new
        /// tunnel session ID.
        pub async fn restart(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/tunnel_sessions/{id}/restart", id = id),
                    reqwest::Method::POST,
                    Some(&types::Item { id: id.into() }),
                )
                .await
        }

        /// Issues a command instructing the ngrok agent that started this tunnel session to
        /// exit.
        pub async fn stop(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/tunnel_sessions/{id}/stop", id = id),
                    reqwest::Method::POST,
                    Some(&types::Item { id: id.into() }),
                )
                .await
        }

        /// Issues a command instructing the ngrok agent to update itself to the latest
        /// version. After this call completes successfully, the ngrok agent will be in the
        /// update process. A caller should wait some amount of time to allow the update to
        /// complete (at least 10 seconds) before making a call to the Restart endpoint to
        /// request that the agent restart itself to start using the new code. This call
        /// will never update an ngrok agent to a new major version which could cause
        /// breaking compatibility issues. If you wish to update to a new major version,
        /// that must be done manually. Still, please be aware that updating your ngrok
        /// agent could break your integration. This call will fail in any of the following
        /// circumstances: there is no update available the ngrok agent's configuration
        /// disabled update checks the agent is currently in process of updating the agent
        /// has already successfully updated but has not yet been restarted
        pub async fn update(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/tunnel_sessions/{id}/update", id = id),
                    reqwest::Method::POST,
                    Some(&types::TunnelSessionsUpdate { id: id.into() }),
                )
                .await
        }
    }
}

pub mod bot_users {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::BotUserList], or of [types::BotUser] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::BotUserList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(self) -> impl Stream<Item = Result<types::BotUserList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::BotUser] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn bot_users(self) -> impl Stream<Item = Result<types::BotUser, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.bot_users.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new bot user
        pub async fn create(&self, req: &types::BotUserCreate) -> Result<types::BotUser, Error> {
            self.c
                .make_request("/bot_users", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete a bot user by ID
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/bot_users/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get the details of a Bot User by ID.
        pub async fn get(&self, id: &str) -> Result<types::BotUser, Error> {
            self.c
                .make_request(
                    &format!("/bot_users/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::BotUserList, Error> {
            self.c
                .make_request("/bot_users", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all bot users in this account.
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of a bot user by ID.
        pub async fn update(&self, req: &types::BotUserUpdate) -> Result<types::BotUser, Error> {
            self.c
                .make_request(
                    &format!("/bot_users/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// Certificate Authorities are x509 certificates that are used to sign other
///  x509 certificates. Attach a Certificate Authority to the Mutual TLS module
///  to verify that the TLS certificate presented by a client has been signed by
///  this CA. Certificate Authorities  are used only for mTLS validation only and
///  thus a private key is not included in the resource.
pub mod certificate_authorities {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// Certificate Authorities are x509 certificates that are used to sign other
    ///  x509 certificates. Attach a Certificate Authority to the Mutual TLS module
    ///  to verify that the TLS certificate presented by a client has been signed by
    ///  this CA. Certificate Authorities  are used only for mTLS validation only and
    ///  thus a private key is not included in the resource.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::CertificateAuthorityList], or of [types::CertificateAuthority] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::CertificateAuthorityList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::CertificateAuthorityList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::CertificateAuthority] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn certificate_authorities(
            self,
        ) -> impl Stream<Item = Result<types::CertificateAuthority, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| {
                    futures::stream::iter(page.certificate_authorities.into_iter().map(Ok))
                })
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Upload a new Certificate Authority
        pub async fn create(
            &self,
            req: &types::CertificateAuthorityCreate,
        ) -> Result<types::CertificateAuthority, Error> {
            self.c
                .make_request("/certificate_authorities", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete a Certificate Authority
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/certificate_authorities/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about a certficate authority
        pub async fn get(&self, id: &str) -> Result<types::CertificateAuthority, Error> {
            self.c
                .make_request(
                    &format!("/certificate_authorities/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::CertificateAuthorityList, Error> {
            self.c
                .make_request("/certificate_authorities", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all Certificate Authority on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of a Certificate Authority by ID
        pub async fn update(
            &self,
            req: &types::CertificateAuthorityUpdate,
        ) -> Result<types::CertificateAuthority, Error> {
            self.c
                .make_request(
                    &format!("/certificate_authorities/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// Tunnel Credentials are ngrok agent authtokens. They authorize the ngrok
///  agent to connect the ngrok service as your account. They are installed with
///  the `ngrok config add-authtoken` command or by specifying it in the `ngrok.yml`
///  configuration file with the `authtoken` property.
pub mod credentials {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// Tunnel Credentials are ngrok agent authtokens. They authorize the ngrok
    ///  agent to connect the ngrok service as your account. They are installed with
    ///  the `ngrok config add-authtoken` command or by specifying it in the `ngrok.yml`
    ///  configuration file with the `authtoken` property.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::CredentialList], or of [types::Credential] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::CredentialList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::CredentialList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::Credential] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn credentials(
            self,
        ) -> impl Stream<Item = Result<types::Credential, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.credentials.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new tunnel authtoken credential. This authtoken credential can be used
        /// to start a new tunnel session. The response to this API call is the only time
        /// the generated token is available. If you need it for future use, you must save
        /// it securely yourself.
        pub async fn create(
            &self,
            req: &types::CredentialCreate,
        ) -> Result<types::Credential, Error> {
            self.c
                .make_request("/credentials", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete a tunnel authtoken credential by ID
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/credentials/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about a tunnel authtoken credential
        pub async fn get(&self, id: &str) -> Result<types::Credential, Error> {
            self.c
                .make_request(
                    &format!("/credentials/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::CredentialList, Error> {
            self.c
                .make_request("/credentials", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all tunnel authtoken credentials on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of an tunnel authtoken credential by ID
        pub async fn update(
            &self,
            req: &types::CredentialUpdate,
        ) -> Result<types::Credential, Error> {
            self.c
                .make_request(
                    &format!("/credentials/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// Endpoints provides an API for querying the endpoint objects
///  which define what tunnel or edge is used to serve a hostport.
///  Only active endpoints associated with a tunnel or backend are returned.
pub mod endpoints {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// Endpoints provides an API for querying the endpoint objects
    ///  which define what tunnel or edge is used to serve a hostport.
    ///  Only active endpoints associated with a tunnel or backend are returned.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::EndpointList], or of [types::Endpoint] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::EndpointList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(self) -> impl Stream<Item = Result<types::EndpointList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::Endpoint] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn endpoints(self) -> impl Stream<Item = Result<types::Endpoint, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.endpoints.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create an endpoint, currently available only for cloud endpoints
        pub async fn create(&self, req: &types::EndpointCreate) -> Result<types::Endpoint, Error> {
            self.c
                .make_request("/endpoints", reqwest::Method::POST, Some(req))
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::EndpointList, Error> {
            self.c
                .make_request("/endpoints", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all active endpoints on the account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Get the status of an endpoint by ID
        pub async fn get(&self, id: &str) -> Result<types::Endpoint, Error> {
            self.c
                .make_request(
                    &format!("/endpoints/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Update an Endpoint by ID, currently available only for cloud endpoints
        pub async fn update(&self, req: &types::EndpointUpdate) -> Result<types::Endpoint, Error> {
            self.c
                .make_request(
                    &format!("/endpoints/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }

        /// Delete an Endpoint by ID, currently available only for cloud endpoints
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/endpoints/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }
    }
}

pub mod event_destinations {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::EventDestinationList], or of [types::EventDestination] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::EventDestinationList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::EventDestinationList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::EventDestination] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn event_destinations(
            self,
        ) -> impl Stream<Item = Result<types::EventDestination, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.event_destinations.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new Event Destination. It will not apply to anything until it is
        /// associated with an Event Subscription.
        pub async fn create(
            &self,
            req: &types::EventDestinationCreate,
        ) -> Result<types::EventDestination, Error> {
            self.c
                .make_request("/event_destinations", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an Event Destination. If the Event Destination is still referenced by an
        /// Event Subscription.
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/event_destinations/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about an Event Destination by ID.
        pub async fn get(&self, id: &str) -> Result<types::EventDestination, Error> {
            self.c
                .make_request(
                    &format!("/event_destinations/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::EventDestinationList, Error> {
            self.c
                .make_request("/event_destinations", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all Event Destinations on this account.
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of an Event Destination.
        pub async fn update(
            &self,
            req: &types::EventDestinationUpdate,
        ) -> Result<types::EventDestination, Error> {
            self.c
                .make_request(
                    &format!("/event_destinations/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

pub mod event_subscriptions {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::EventSubscriptionList], or of [types::EventSubscription] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::EventSubscriptionList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::EventSubscriptionList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::EventSubscription] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn event_subscriptions(
            self,
        ) -> impl Stream<Item = Result<types::EventSubscription, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.event_subscriptions.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create an Event Subscription.
        pub async fn create(
            &self,
            req: &types::EventSubscriptionCreate,
        ) -> Result<types::EventSubscription, Error> {
            self.c
                .make_request("/event_subscriptions", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an Event Subscription.
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/event_subscriptions/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get an Event Subscription by ID.
        pub async fn get(&self, id: &str) -> Result<types::EventSubscription, Error> {
            self.c
                .make_request(
                    &format!("/event_subscriptions/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::EventSubscriptionList, Error> {
            self.c
                .make_request("/event_subscriptions", reqwest::Method::GET, Some(req))
                .await
        }

        /// List this Account's Event Subscriptions.
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update an Event Subscription.
        pub async fn update(
            &self,
            req: &types::EventSubscriptionUpdate,
        ) -> Result<types::EventSubscription, Error> {
            self.c
                .make_request(
                    &format!("/event_subscriptions/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

pub mod event_sources {
    use crate::types;
    use crate::Error;

    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Add an additional type for which this event subscription will trigger
        pub async fn create(
            &self,
            req: &types::EventSourceCreate,
        ) -> Result<types::EventSource, Error> {
            self.c
                .make_request(
                    &format!(
                        "/event_subscriptions/{subscription_id}/sources",
                        subscription_id = req.subscription_id
                    ),
                    reqwest::Method::POST,
                    Some(req),
                )
                .await
        }

        /// Remove a type for which this event subscription will trigger
        pub async fn delete(&self, req: &types::EventSourceItem) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!(
                        "/event_subscriptions/{subscription_id}/sources/{type}",
                        subscription_id = req.subscription_id,
                        r#type = req.r#type
                    ),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get the details for a given type that triggers for the given event subscription
        pub async fn get(&self, req: &types::EventSourceItem) -> Result<types::EventSource, Error> {
            self.c
                .make_request(
                    &format!(
                        "/event_subscriptions/{subscription_id}/sources/{type}",
                        subscription_id = req.subscription_id,
                        r#type = req.r#type
                    ),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// List the types for which this event subscription will trigger
        pub async fn list(&self, subscription_id: &str) -> Result<types::EventSourceList, Error> {
            self.c
                .make_request(
                    &format!(
                        "/event_subscriptions/{subscription_id}/sources",
                        subscription_id = subscription_id
                    ),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Update the type for which this event subscription will trigger
        pub async fn update(
            &self,
            req: &types::EventSourceUpdate,
        ) -> Result<types::EventSource, Error> {
            self.c
                .make_request(
                    &format!(
                        "/event_subscriptions/{subscription_id}/sources/{type}",
                        subscription_id = req.subscription_id,
                        r#type = req.r#type
                    ),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// IP Policies are reusable groups of CIDR ranges with an `allow` or `deny`
///  action. They can be attached to endpoints via the Endpoint Configuration IP
///  Policy module. They can also be used with IP Restrictions to control source
///  IP ranges that can start tunnel sessions and connect to the API and dashboard.
pub mod ip_policies {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// IP Policies are reusable groups of CIDR ranges with an `allow` or `deny`
    ///  action. They can be attached to endpoints via the Endpoint Configuration IP
    ///  Policy module. They can also be used with IP Restrictions to control source
    ///  IP ranges that can start tunnel sessions and connect to the API and dashboard.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::IPPolicyList], or of [types::IPPolicy] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::IPPolicyList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(self) -> impl Stream<Item = Result<types::IPPolicyList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::IPPolicy] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn ip_policies(
            self,
        ) -> impl Stream<Item = Result<types::IPPolicy, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.ip_policies.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new IP policy. It will not apply to any traffic until you associate to
        /// a traffic source via an endpoint configuration or IP restriction.
        pub async fn create(&self, req: &types::IPPolicyCreate) -> Result<types::IPPolicy, Error> {
            self.c
                .make_request("/ip_policies", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an IP policy. If the IP policy is referenced by another object for the
        /// purposes of traffic restriction it will be treated as if the IP policy remains
        /// but has zero rules.
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/ip_policies/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about an IP policy by ID.
        pub async fn get(&self, id: &str) -> Result<types::IPPolicy, Error> {
            self.c
                .make_request(
                    &format!("/ip_policies/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::IPPolicyList, Error> {
            self.c
                .make_request("/ip_policies", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all IP policies on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of an IP policy by ID
        pub async fn update(&self, req: &types::IPPolicyUpdate) -> Result<types::IPPolicy, Error> {
            self.c
                .make_request(
                    &format!("/ip_policies/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// IP Policy Rules are the IPv4 or IPv6 CIDRs entries that
///  make up an IP Policy.
pub mod ip_policy_rules {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// IP Policy Rules are the IPv4 or IPv6 CIDRs entries that
    ///  make up an IP Policy.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::IPPolicyRuleList], or of [types::IPPolicyRule] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::IPPolicyRuleList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::IPPolicyRuleList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::IPPolicyRule] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn ip_policy_rules(
            self,
        ) -> impl Stream<Item = Result<types::IPPolicyRule, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.ip_policy_rules.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new IP policy rule attached to an IP Policy.
        pub async fn create(
            &self,
            req: &types::IPPolicyRuleCreate,
        ) -> Result<types::IPPolicyRule, Error> {
            self.c
                .make_request("/ip_policy_rules", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an IP policy rule.
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/ip_policy_rules/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about an IP policy rule by ID.
        pub async fn get(&self, id: &str) -> Result<types::IPPolicyRule, Error> {
            self.c
                .make_request(
                    &format!("/ip_policy_rules/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::IPPolicyRuleList, Error> {
            self.c
                .make_request("/ip_policy_rules", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all IP policy rules on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of an IP policy rule by ID
        pub async fn update(
            &self,
            req: &types::IPPolicyRuleUpdate,
        ) -> Result<types::IPPolicyRule, Error> {
            self.c
                .make_request(
                    &format!("/ip_policy_rules/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// An IP restriction is a restriction placed on the CIDRs that are allowed to
///  initiate traffic to a specific aspect of your ngrok account. An IP
///  restriction has a type which defines the ingress it applies to. IP
///  restrictions can be used to enforce the source IPs that can make API
///  requests, log in to the dashboard, start ngrok agents, and connect to your
///  public-facing endpoints.
pub mod ip_restrictions {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// An IP restriction is a restriction placed on the CIDRs that are allowed to
    ///  initiate traffic to a specific aspect of your ngrok account. An IP
    ///  restriction has a type which defines the ingress it applies to. IP
    ///  restrictions can be used to enforce the source IPs that can make API
    ///  requests, log in to the dashboard, start ngrok agents, and connect to your
    ///  public-facing endpoints.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::IPRestrictionList], or of [types::IPRestriction] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::IPRestrictionList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::IPRestrictionList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::IPRestriction] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn ip_restrictions(
            self,
        ) -> impl Stream<Item = Result<types::IPRestriction, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.ip_restrictions.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new IP restriction
        pub async fn create(
            &self,
            req: &types::IPRestrictionCreate,
        ) -> Result<types::IPRestriction, Error> {
            self.c
                .make_request("/ip_restrictions", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an IP restriction
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/ip_restrictions/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about an IP restriction
        pub async fn get(&self, id: &str) -> Result<types::IPRestriction, Error> {
            self.c
                .make_request(
                    &format!("/ip_restrictions/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::IPRestrictionList, Error> {
            self.c
                .make_request("/ip_restrictions", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all IP restrictions on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of an IP restriction by ID
        pub async fn update(
            &self,
            req: &types::IPRestrictionUpdate,
        ) -> Result<types::IPRestriction, Error> {
            self.c
                .make_request(
                    &format!("/ip_restrictions/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// KubernetesOperators is used by the Kubernetes Operator to register and
///  manage its own resource, as well as for users to see active kubernetes
///  clusters.
pub mod kubernetes_operators {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// KubernetesOperators is used by the Kubernetes Operator to register and
    ///  manage its own resource, as well as for users to see active kubernetes
    ///  clusters.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::KubernetesOperatorList], or of [types::KubernetesOperator] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::KubernetesOperatorList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::KubernetesOperatorList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::KubernetesOperator] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn operators(
            self,
        ) -> impl Stream<Item = Result<types::KubernetesOperator, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.operators.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    /// Provides streams of pages of [types::EndpointList], or of [types::Endpoint] directly.
    pub struct GetBoundEndpoints {
        c: std::sync::Arc<Client>,
        req: types::ItemPaging,
    }

    impl GetBoundEndpoints {
        /// Iterate over [types::EndpointList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(self) -> impl Stream<Item = Result<types::EndpointList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::ItemPaging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.get_bound_endpoints_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::Endpoint] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn endpoints(self) -> impl Stream<Item = Result<types::Endpoint, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.endpoints.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new Kubernetes Operator
        pub async fn create(
            &self,
            req: &types::KubernetesOperatorCreate,
        ) -> Result<types::KubernetesOperator, Error> {
            self.c
                .make_request("/kubernetes_operators", reqwest::Method::POST, Some(req))
                .await
        }

        /// Update an existing Kubernetes operator by ID.
        pub async fn update(
            &self,
            req: &types::KubernetesOperatorUpdate,
        ) -> Result<types::KubernetesOperator, Error> {
            self.c
                .make_request(
                    &format!("/kubernetes_operators/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }

        /// Delete a Kubernetes Operator
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/kubernetes_operators/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get of a Kubernetes Operator
        pub async fn get(&self, id: &str) -> Result<types::KubernetesOperator, Error> {
            self.c
                .make_request(
                    &format!("/kubernetes_operators/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::KubernetesOperatorList, Error> {
            self.c
                .make_request("/kubernetes_operators", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all Kubernetes Operators owned by this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Get a single page without pagination. Prefer using get_bound_endpoints
        /// which will do pagination for you.
        pub(crate) async fn get_bound_endpoints_page(
            &self,
            req: &types::ItemPaging,
        ) -> Result<types::EndpointList, Error> {
            self.c
                .make_request(
                    &format!("/kubernetes_operators/{id}/bound_endpoints", id = req.id),
                    reqwest::Method::GET,
                    Some(req),
                )
                .await
        }

        /// List Endpoints bound to a Kubernetes Operator
        pub fn get_bound_endpoints(&self, req: types::ItemPaging) -> GetBoundEndpoints {
            GetBoundEndpoints {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }
    }
}

/// Reserved Addresses are TCP addresses that can be used to listen for traffic.
///  TCP address hostnames and ports are assigned by ngrok, they cannot be
///  chosen.
pub mod reserved_addrs {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// Reserved Addresses are TCP addresses that can be used to listen for traffic.
    ///  TCP address hostnames and ports are assigned by ngrok, they cannot be
    ///  chosen.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::ReservedAddrList], or of [types::ReservedAddr] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::ReservedAddrList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::ReservedAddrList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::ReservedAddr] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn reserved_addrs(
            self,
        ) -> impl Stream<Item = Result<types::ReservedAddr, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.reserved_addrs.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new reserved address.
        pub async fn create(
            &self,
            req: &types::ReservedAddrCreate,
        ) -> Result<types::ReservedAddr, Error> {
            self.c
                .make_request("/reserved_addrs", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete a reserved address.
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/reserved_addrs/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get the details of a reserved address.
        pub async fn get(&self, id: &str) -> Result<types::ReservedAddr, Error> {
            self.c
                .make_request(
                    &format!("/reserved_addrs/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::ReservedAddrList, Error> {
            self.c
                .make_request("/reserved_addrs", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all reserved addresses on this account.
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update the attributes of a reserved address.
        pub async fn update(
            &self,
            req: &types::ReservedAddrUpdate,
        ) -> Result<types::ReservedAddr, Error> {
            self.c
                .make_request(
                    &format!("/reserved_addrs/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// Reserved Domains are hostnames that you can listen for traffic on. Domains
///  can be used to listen for http, https or tls traffic. You may use a domain
///  that you own by creating a CNAME record specified in the returned resource.
///  This CNAME record points traffic for that domain to ngrok's edge servers.
pub mod reserved_domains {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// Reserved Domains are hostnames that you can listen for traffic on. Domains
    ///  can be used to listen for http, https or tls traffic. You may use a domain
    ///  that you own by creating a CNAME record specified in the returned resource.
    ///  This CNAME record points traffic for that domain to ngrok's edge servers.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::ReservedDomainList], or of [types::ReservedDomain] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::ReservedDomainList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::ReservedDomainList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::ReservedDomain] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn reserved_domains(
            self,
        ) -> impl Stream<Item = Result<types::ReservedDomain, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.reserved_domains.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new reserved domain.
        pub async fn create(
            &self,
            req: &types::ReservedDomainCreate,
        ) -> Result<types::ReservedDomain, Error> {
            self.c
                .make_request("/reserved_domains", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete a reserved domain.
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/reserved_domains/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get the details of a reserved domain.
        pub async fn get(&self, id: &str) -> Result<types::ReservedDomain, Error> {
            self.c
                .make_request(
                    &format!("/reserved_domains/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::ReservedDomainList, Error> {
            self.c
                .make_request("/reserved_domains", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all reserved domains on this account.
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update the attributes of a reserved domain.
        pub async fn update(
            &self,
            req: &types::ReservedDomainUpdate,
        ) -> Result<types::ReservedDomain, Error> {
            self.c
                .make_request(
                    &format!("/reserved_domains/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }

        /// Detach the certificate management policy attached to a reserved domain.
        pub async fn delete_certificate_management_policy(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!(
                        "/reserved_domains/{id}/certificate_management_policy",
                        id = id
                    ),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Detach the certificate attached to a reserved domain.
        pub async fn delete_certificate(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/reserved_domains/{id}/certificate", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }
    }
}

/// An SSH Certificate Authority is a pair of an SSH Certificate and its private
///  key that can be used to sign other SSH host and user certificates.
pub mod ssh_certificate_authorities {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// An SSH Certificate Authority is a pair of an SSH Certificate and its private
    ///  key that can be used to sign other SSH host and user certificates.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::SSHCertificateAuthorityList], or of [types::SSHCertificateAuthority] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::SSHCertificateAuthorityList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::SSHCertificateAuthorityList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::SSHCertificateAuthority] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn ssh_certificate_authorities(
            self,
        ) -> impl Stream<Item = Result<types::SSHCertificateAuthority, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| {
                    futures::stream::iter(page.ssh_certificate_authorities.into_iter().map(Ok))
                })
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new SSH Certificate Authority
        pub async fn create(
            &self,
            req: &types::SSHCertificateAuthorityCreate,
        ) -> Result<types::SSHCertificateAuthority, Error> {
            self.c
                .make_request(
                    "/ssh_certificate_authorities",
                    reqwest::Method::POST,
                    Some(req),
                )
                .await
        }

        /// Delete an SSH Certificate Authority
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/ssh_certificate_authorities/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about an SSH Certficate Authority
        pub async fn get(&self, id: &str) -> Result<types::SSHCertificateAuthority, Error> {
            self.c
                .make_request(
                    &format!("/ssh_certificate_authorities/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::SSHCertificateAuthorityList, Error> {
            self.c
                .make_request(
                    "/ssh_certificate_authorities",
                    reqwest::Method::GET,
                    Some(req),
                )
                .await
        }

        /// List all SSH Certificate Authorities on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update an SSH Certificate Authority
        pub async fn update(
            &self,
            req: &types::SSHCertificateAuthorityUpdate,
        ) -> Result<types::SSHCertificateAuthority, Error> {
            self.c
                .make_request(
                    &format!("/ssh_certificate_authorities/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// SSH Credentials are SSH public keys that can be used to start SSH tunnels
///  via the ngrok SSH tunnel gateway.
pub mod ssh_credentials {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// SSH Credentials are SSH public keys that can be used to start SSH tunnels
    ///  via the ngrok SSH tunnel gateway.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::SSHCredentialList], or of [types::SSHCredential] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::SSHCredentialList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::SSHCredentialList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::SSHCredential] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn ssh_credentials(
            self,
        ) -> impl Stream<Item = Result<types::SSHCredential, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.ssh_credentials.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new ssh_credential from an uploaded public SSH key. This ssh credential
        /// can be used to start new tunnels via ngrok's SSH gateway.
        pub async fn create(
            &self,
            req: &types::SSHCredentialCreate,
        ) -> Result<types::SSHCredential, Error> {
            self.c
                .make_request("/ssh_credentials", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an ssh_credential by ID
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/ssh_credentials/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about an ssh_credential
        pub async fn get(&self, id: &str) -> Result<types::SSHCredential, Error> {
            self.c
                .make_request(
                    &format!("/ssh_credentials/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::SSHCredentialList, Error> {
            self.c
                .make_request("/ssh_credentials", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all ssh credentials on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of an ssh_credential by ID
        pub async fn update(
            &self,
            req: &types::SSHCredentialUpdate,
        ) -> Result<types::SSHCredential, Error> {
            self.c
                .make_request(
                    &format!("/ssh_credentials/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// SSH Host Certificates along with the corresponding private key allows an SSH
///  server to assert its authenticity to connecting SSH clients who trust the
///  SSH Certificate Authority that was used to sign the certificate.
pub mod ssh_host_certificates {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// SSH Host Certificates along with the corresponding private key allows an SSH
    ///  server to assert its authenticity to connecting SSH clients who trust the
    ///  SSH Certificate Authority that was used to sign the certificate.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::SSHHostCertificateList], or of [types::SSHHostCertificate] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::SSHHostCertificateList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::SSHHostCertificateList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::SSHHostCertificate] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn ssh_host_certificates(
            self,
        ) -> impl Stream<Item = Result<types::SSHHostCertificate, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| {
                    futures::stream::iter(page.ssh_host_certificates.into_iter().map(Ok))
                })
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new SSH Host Certificate
        pub async fn create(
            &self,
            req: &types::SSHHostCertificateCreate,
        ) -> Result<types::SSHHostCertificate, Error> {
            self.c
                .make_request("/ssh_host_certificates", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an SSH Host Certificate
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/ssh_host_certificates/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about an SSH Host Certficate
        pub async fn get(&self, id: &str) -> Result<types::SSHHostCertificate, Error> {
            self.c
                .make_request(
                    &format!("/ssh_host_certificates/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::SSHHostCertificateList, Error> {
            self.c
                .make_request("/ssh_host_certificates", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all SSH Host Certificates issued on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update an SSH Host Certificate
        pub async fn update(
            &self,
            req: &types::SSHHostCertificateUpdate,
        ) -> Result<types::SSHHostCertificate, Error> {
            self.c
                .make_request(
                    &format!("/ssh_host_certificates/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// SSH User Certificates are presented by SSH clients when connecting to an SSH
///  server to authenticate their connection. The SSH server must trust the SSH
///  Certificate Authority used to sign the certificate.
pub mod ssh_user_certificates {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// SSH User Certificates are presented by SSH clients when connecting to an SSH
    ///  server to authenticate their connection. The SSH server must trust the SSH
    ///  Certificate Authority used to sign the certificate.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::SSHUserCertificateList], or of [types::SSHUserCertificate] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::SSHUserCertificateList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::SSHUserCertificateList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::SSHUserCertificate] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn ssh_user_certificates(
            self,
        ) -> impl Stream<Item = Result<types::SSHUserCertificate, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| {
                    futures::stream::iter(page.ssh_user_certificates.into_iter().map(Ok))
                })
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Create a new SSH User Certificate
        pub async fn create(
            &self,
            req: &types::SSHUserCertificateCreate,
        ) -> Result<types::SSHUserCertificate, Error> {
            self.c
                .make_request("/ssh_user_certificates", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete an SSH User Certificate
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/ssh_user_certificates/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about an SSH User Certficate
        pub async fn get(&self, id: &str) -> Result<types::SSHUserCertificate, Error> {
            self.c
                .make_request(
                    &format!("/ssh_user_certificates/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::SSHUserCertificateList, Error> {
            self.c
                .make_request("/ssh_user_certificates", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all SSH User Certificates issued on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update an SSH User Certificate
        pub async fn update(
            &self,
            req: &types::SSHUserCertificateUpdate,
        ) -> Result<types::SSHUserCertificate, Error> {
            self.c
                .make_request(
                    &format!("/ssh_user_certificates/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// TLS Certificates are pairs of x509 certificates and their matching private
///  key that can be used to terminate TLS traffic. TLS certificates are unused
///  until they are attached to a Domain. TLS Certificates may also be
///  provisioned by ngrok automatically for domains on which you have enabled
///  automated certificate provisioning.
pub mod tls_certificates {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// TLS Certificates are pairs of x509 certificates and their matching private
    ///  key that can be used to terminate TLS traffic. TLS certificates are unused
    ///  until they are attached to a Domain. TLS Certificates may also be
    ///  provisioned by ngrok automatically for domains on which you have enabled
    ///  automated certificate provisioning.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::TLSCertificateList], or of [types::TLSCertificate] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::TLSCertificateList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(
            self,
        ) -> impl Stream<Item = Result<types::TLSCertificateList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::TLSCertificate] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn tls_certificates(
            self,
        ) -> impl Stream<Item = Result<types::TLSCertificate, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.tls_certificates.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Upload a new TLS certificate
        pub async fn create(
            &self,
            req: &types::TLSCertificateCreate,
        ) -> Result<types::TLSCertificate, Error> {
            self.c
                .make_request("/tls_certificates", reqwest::Method::POST, Some(req))
                .await
        }

        /// Delete a TLS certificate
        pub async fn delete(&self, id: &str) -> Result<(), Error> {
            self.c
                .make_request(
                    &format!("/tls_certificates/{id}", id = id),
                    reqwest::Method::DELETE,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get detailed information about a TLS certificate
        pub async fn get(&self, id: &str) -> Result<types::TLSCertificate, Error> {
            self.c
                .make_request(
                    &format!("/tls_certificates/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::TLSCertificateList, Error> {
            self.c
                .make_request("/tls_certificates", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all TLS certificates on this account
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Update attributes of a TLS Certificate by ID
        pub async fn update(
            &self,
            req: &types::TLSCertificateUpdate,
        ) -> Result<types::TLSCertificate, Error> {
            self.c
                .make_request(
                    &format!("/tls_certificates/{id}", id = req.id),
                    reqwest::Method::PATCH,
                    Some(req),
                )
                .await
        }
    }
}

/// Tunnels provide endpoints to access services exposed by a running ngrok
///  agent tunnel session or an SSH reverse tunnel session.
pub mod tunnels {
    use crate::types;
    use crate::Error;
    use futures::{Stream, TryStreamExt};

    /// Tunnels provide endpoints to access services exposed by a running ngrok
    ///  agent tunnel session or an SSH reverse tunnel session.
    #[derive(Debug, Clone)]
    pub struct Client {
        c: crate::Client,
    }
    /// Provides streams of pages of [types::TunnelList], or of [types::Tunnel] directly.
    pub struct List {
        c: std::sync::Arc<Client>,
        req: types::Paging,
    }

    impl List {
        /// Iterate over [types::TunnelList].
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn pages(self) -> impl Stream<Item = Result<types::TunnelList, Error>> + Unpin {
            struct State {
                c: std::sync::Arc<Client>,
                req: types::Paging,
                init: bool,
                cur_uri: Option<String>,
            }

            let s = State {
                c: self.c,
                req: self.req,
                init: true,
                cur_uri: None,
            };

            Box::pin(futures::stream::unfold(s, |s| async move {
                let page_resp = match (s.init, &s.cur_uri) {
                    (true, _) => s.c.list_page(&s.req).await,
                    (false, None) => {
                        return None;
                    }
                    (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

        /// Iterate over [types::Tunnel] items.
        ///
        /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
        /// documentation for more info on usage.
        pub async fn tunnels(self) -> impl Stream<Item = Result<types::Tunnel, Error>> + Unpin {
            self.pages()
                .await
                .map_ok(|page| futures::stream::iter(page.tunnels.into_iter().map(Ok)))
                .try_flatten()
        }
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }

        /// Get a single page without pagination. Prefer using list
        /// which will do pagination for you.
        pub(crate) async fn list_page(
            &self,
            req: &types::Paging,
        ) -> Result<types::TunnelList, Error> {
            self.c
                .make_request("/tunnels", reqwest::Method::GET, Some(req))
                .await
        }

        /// List all online tunnels currently running on the account.
        pub fn list(&self, req: types::Paging) -> List {
            List {
                c: std::sync::Arc::new(self.clone()),
                req,
            }
        }

        /// Get the status of a tunnel by ID
        pub async fn get(&self, id: &str) -> Result<types::Tunnel, Error> {
            self.c
                .make_request(
                    &format!("/tunnels/{id}", id = id),
                    reqwest::Method::GET,
                    None::<Option<()>>,
                )
                .await
        }
    }
}
impl Client {
    pub fn abuse_reports(&self) -> abuse_reports::Client {
        abuse_reports::Client::new(self.clone())
    }
    pub fn agent_ingresses(&self) -> agent_ingresses::Client {
        agent_ingresses::Client::new(self.clone())
    }
    pub fn api_keys(&self) -> api_keys::Client {
        api_keys::Client::new(self.clone())
    }
    pub fn application_sessions(&self) -> application_sessions::Client {
        application_sessions::Client::new(self.clone())
    }
    pub fn application_users(&self) -> application_users::Client {
        application_users::Client::new(self.clone())
    }
    pub fn tunnel_sessions(&self) -> tunnel_sessions::Client {
        tunnel_sessions::Client::new(self.clone())
    }
    pub fn bot_users(&self) -> bot_users::Client {
        bot_users::Client::new(self.clone())
    }
    pub fn certificate_authorities(&self) -> certificate_authorities::Client {
        certificate_authorities::Client::new(self.clone())
    }
    pub fn credentials(&self) -> credentials::Client {
        credentials::Client::new(self.clone())
    }
    pub fn endpoints(&self) -> endpoints::Client {
        endpoints::Client::new(self.clone())
    }
    pub fn event_destinations(&self) -> event_destinations::Client {
        event_destinations::Client::new(self.clone())
    }
    pub fn event_subscriptions(&self) -> event_subscriptions::Client {
        event_subscriptions::Client::new(self.clone())
    }
    pub fn event_sources(&self) -> event_sources::Client {
        event_sources::Client::new(self.clone())
    }
    pub fn ip_policies(&self) -> ip_policies::Client {
        ip_policies::Client::new(self.clone())
    }
    pub fn ip_policy_rules(&self) -> ip_policy_rules::Client {
        ip_policy_rules::Client::new(self.clone())
    }
    pub fn ip_restrictions(&self) -> ip_restrictions::Client {
        ip_restrictions::Client::new(self.clone())
    }
    pub fn kubernetes_operators(&self) -> kubernetes_operators::Client {
        kubernetes_operators::Client::new(self.clone())
    }
    pub fn reserved_addrs(&self) -> reserved_addrs::Client {
        reserved_addrs::Client::new(self.clone())
    }
    pub fn reserved_domains(&self) -> reserved_domains::Client {
        reserved_domains::Client::new(self.clone())
    }
    pub fn ssh_certificate_authorities(&self) -> ssh_certificate_authorities::Client {
        ssh_certificate_authorities::Client::new(self.clone())
    }
    pub fn ssh_credentials(&self) -> ssh_credentials::Client {
        ssh_credentials::Client::new(self.clone())
    }
    pub fn ssh_host_certificates(&self) -> ssh_host_certificates::Client {
        ssh_host_certificates::Client::new(self.clone())
    }
    pub fn ssh_user_certificates(&self) -> ssh_user_certificates::Client {
        ssh_user_certificates::Client::new(self.clone())
    }
    pub fn tls_certificates(&self) -> tls_certificates::Client {
        tls_certificates::Client::new(self.clone())
    }
    pub fn tunnels(&self) -> tunnels::Client {
        tunnels::Client::new(self.clone())
    }
}

pub mod backends {
    pub struct Client {
        c: crate::Client,
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }
    }

    /// A Failover backend defines failover behavior within a list of referenced
    ///  backends. Traffic is sent to the first backend in the list. If that backend
    ///  is offline or no connection can be established, ngrok attempts to connect to
    ///  the next backend in the list until one is successful.
    pub mod failover {
        use crate::types;
        use crate::Error;
        use futures::{Stream, TryStreamExt};

        /// A Failover backend defines failover behavior within a list of referenced
        ///  backends. Traffic is sent to the first backend in the list. If that backend
        ///  is offline or no connection can be established, ngrok attempts to connect to
        ///  the next backend in the list until one is successful.
        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }
        /// Provides streams of pages of [types::FailoverBackendList], or of [types::FailoverBackend] directly.
        pub struct List {
            c: std::sync::Arc<Client>,
            req: types::Paging,
        }

        impl List {
            /// Iterate over [types::FailoverBackendList].
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn pages(
                self,
            ) -> impl Stream<Item = Result<types::FailoverBackendList, Error>> + Unpin {
                struct State {
                    c: std::sync::Arc<Client>,
                    req: types::Paging,
                    init: bool,
                    cur_uri: Option<String>,
                }

                let s = State {
                    c: self.c,
                    req: self.req,
                    init: true,
                    cur_uri: None,
                };

                Box::pin(futures::stream::unfold(s, |s| async move {
                    let page_resp = match (s.init, &s.cur_uri) {
                        (true, _) => s.c.list_page(&s.req).await,
                        (false, None) => {
                            return None;
                        }
                        (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

            /// Iterate over [types::FailoverBackend] items.
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn backends(
                self,
            ) -> impl Stream<Item = Result<types::FailoverBackend, Error>> + Unpin {
                self.pages()
                    .await
                    .map_ok(|page| futures::stream::iter(page.backends.into_iter().map(Ok)))
                    .try_flatten()
            }
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            /// Create a new Failover backend
            pub async fn create(
                &self,
                req: &types::FailoverBackendCreate,
            ) -> Result<types::FailoverBackend, Error> {
                self.c
                    .make_request("/backends/failover", reqwest::Method::POST, Some(req))
                    .await
            }

            /// Delete a Failover backend by ID.
            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/backends/failover/{id}", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get detailed information about a Failover backend by ID
            pub async fn get(&self, id: &str) -> Result<types::FailoverBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/failover/{id}", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get a single page without pagination. Prefer using list
            /// which will do pagination for you.
            pub(crate) async fn list_page(
                &self,
                req: &types::Paging,
            ) -> Result<types::FailoverBackendList, Error> {
                self.c
                    .make_request("/backends/failover", reqwest::Method::GET, Some(req))
                    .await
            }

            /// List all Failover backends on this account
            pub fn list(&self, req: types::Paging) -> List {
                List {
                    c: std::sync::Arc::new(self.clone()),
                    req,
                }
            }

            /// Update Failover backend by ID
            pub async fn update(
                &self,
                req: &types::FailoverBackendUpdate,
            ) -> Result<types::FailoverBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/failover/{id}", id = req.id),
                        reqwest::Method::PATCH,
                        Some(req),
                    )
                    .await
            }
        }
    }

    pub mod http_response {
        use crate::types;
        use crate::Error;
        use futures::{Stream, TryStreamExt};

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }
        /// Provides streams of pages of [types::HTTPResponseBackendList], or of [types::HTTPResponseBackend] directly.
        pub struct List {
            c: std::sync::Arc<Client>,
            req: types::Paging,
        }

        impl List {
            /// Iterate over [types::HTTPResponseBackendList].
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn pages(
                self,
            ) -> impl Stream<Item = Result<types::HTTPResponseBackendList, Error>> + Unpin
            {
                struct State {
                    c: std::sync::Arc<Client>,
                    req: types::Paging,
                    init: bool,
                    cur_uri: Option<String>,
                }

                let s = State {
                    c: self.c,
                    req: self.req,
                    init: true,
                    cur_uri: None,
                };

                Box::pin(futures::stream::unfold(s, |s| async move {
                    let page_resp = match (s.init, &s.cur_uri) {
                        (true, _) => s.c.list_page(&s.req).await,
                        (false, None) => {
                            return None;
                        }
                        (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

            /// Iterate over [types::HTTPResponseBackend] items.
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn backends(
                self,
            ) -> impl Stream<Item = Result<types::HTTPResponseBackend, Error>> + Unpin {
                self.pages()
                    .await
                    .map_ok(|page| futures::stream::iter(page.backends.into_iter().map(Ok)))
                    .try_flatten()
            }
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn create(
                &self,
                req: &types::HTTPResponseBackendCreate,
            ) -> Result<types::HTTPResponseBackend, Error> {
                self.c
                    .make_request("/backends/http_response", reqwest::Method::POST, Some(req))
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/backends/http_response/{id}", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::HTTPResponseBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/http_response/{id}", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get a single page without pagination. Prefer using list
            /// which will do pagination for you.
            pub(crate) async fn list_page(
                &self,
                req: &types::Paging,
            ) -> Result<types::HTTPResponseBackendList, Error> {
                self.c
                    .make_request("/backends/http_response", reqwest::Method::GET, Some(req))
                    .await
            }

            pub fn list(&self, req: types::Paging) -> List {
                List {
                    c: std::sync::Arc::new(self.clone()),
                    req,
                }
            }

            pub async fn update(
                &self,
                req: &types::HTTPResponseBackendUpdate,
            ) -> Result<types::HTTPResponseBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/http_response/{id}", id = req.id),
                        reqwest::Method::PATCH,
                        Some(req),
                    )
                    .await
            }
        }
    }

    /// A static backend sends traffic to a TCP address (hostname and port) that
    ///  is reachable on the public internet.
    pub mod static_address {
        use crate::types;
        use crate::Error;
        use futures::{Stream, TryStreamExt};

        /// A static backend sends traffic to a TCP address (hostname and port) that
        ///  is reachable on the public internet.
        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }
        /// Provides streams of pages of [types::StaticBackendList], or of [types::StaticBackend] directly.
        pub struct List {
            c: std::sync::Arc<Client>,
            req: types::Paging,
        }

        impl List {
            /// Iterate over [types::StaticBackendList].
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn pages(
                self,
            ) -> impl Stream<Item = Result<types::StaticBackendList, Error>> + Unpin {
                struct State {
                    c: std::sync::Arc<Client>,
                    req: types::Paging,
                    init: bool,
                    cur_uri: Option<String>,
                }

                let s = State {
                    c: self.c,
                    req: self.req,
                    init: true,
                    cur_uri: None,
                };

                Box::pin(futures::stream::unfold(s, |s| async move {
                    let page_resp = match (s.init, &s.cur_uri) {
                        (true, _) => s.c.list_page(&s.req).await,
                        (false, None) => {
                            return None;
                        }
                        (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

            /// Iterate over [types::StaticBackend] items.
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn backends(
                self,
            ) -> impl Stream<Item = Result<types::StaticBackend, Error>> + Unpin {
                self.pages()
                    .await
                    .map_ok(|page| futures::stream::iter(page.backends.into_iter().map(Ok)))
                    .try_flatten()
            }
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            /// Create a new static backend
            pub async fn create(
                &self,
                req: &types::StaticBackendCreate,
            ) -> Result<types::StaticBackend, Error> {
                self.c
                    .make_request("/backends/static", reqwest::Method::POST, Some(req))
                    .await
            }

            /// Delete a static backend by ID.
            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/backends/static/{id}", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get detailed information about a static backend by ID
            pub async fn get(&self, id: &str) -> Result<types::StaticBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/static/{id}", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get a single page without pagination. Prefer using list
            /// which will do pagination for you.
            pub(crate) async fn list_page(
                &self,
                req: &types::Paging,
            ) -> Result<types::StaticBackendList, Error> {
                self.c
                    .make_request("/backends/static", reqwest::Method::GET, Some(req))
                    .await
            }

            /// List all static backends on this account
            pub fn list(&self, req: types::Paging) -> List {
                List {
                    c: std::sync::Arc::new(self.clone()),
                    req,
                }
            }

            /// Update static backend by ID
            pub async fn update(
                &self,
                req: &types::StaticBackendUpdate,
            ) -> Result<types::StaticBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/static/{id}", id = req.id),
                        reqwest::Method::PATCH,
                        Some(req),
                    )
                    .await
            }
        }
    }

    /// A Tunnel Group Backend balances traffic among all online tunnels that match
    ///  a label selector.
    pub mod tunnel_group {
        use crate::types;
        use crate::Error;
        use futures::{Stream, TryStreamExt};

        /// A Tunnel Group Backend balances traffic among all online tunnels that match
        ///  a label selector.
        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }
        /// Provides streams of pages of [types::TunnelGroupBackendList], or of [types::TunnelGroupBackend] directly.
        pub struct List {
            c: std::sync::Arc<Client>,
            req: types::Paging,
        }

        impl List {
            /// Iterate over [types::TunnelGroupBackendList].
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn pages(
                self,
            ) -> impl Stream<Item = Result<types::TunnelGroupBackendList, Error>> + Unpin
            {
                struct State {
                    c: std::sync::Arc<Client>,
                    req: types::Paging,
                    init: bool,
                    cur_uri: Option<String>,
                }

                let s = State {
                    c: self.c,
                    req: self.req,
                    init: true,
                    cur_uri: None,
                };

                Box::pin(futures::stream::unfold(s, |s| async move {
                    let page_resp = match (s.init, &s.cur_uri) {
                        (true, _) => s.c.list_page(&s.req).await,
                        (false, None) => {
                            return None;
                        }
                        (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

            /// Iterate over [types::TunnelGroupBackend] items.
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn backends(
                self,
            ) -> impl Stream<Item = Result<types::TunnelGroupBackend, Error>> + Unpin {
                self.pages()
                    .await
                    .map_ok(|page| futures::stream::iter(page.backends.into_iter().map(Ok)))
                    .try_flatten()
            }
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            /// Create a new TunnelGroup backend
            pub async fn create(
                &self,
                req: &types::TunnelGroupBackendCreate,
            ) -> Result<types::TunnelGroupBackend, Error> {
                self.c
                    .make_request("/backends/tunnel_group", reqwest::Method::POST, Some(req))
                    .await
            }

            /// Delete a TunnelGroup backend by ID.
            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/backends/tunnel_group/{id}", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get detailed information about a TunnelGroup backend by ID
            pub async fn get(&self, id: &str) -> Result<types::TunnelGroupBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/tunnel_group/{id}", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get a single page without pagination. Prefer using list
            /// which will do pagination for you.
            pub(crate) async fn list_page(
                &self,
                req: &types::Paging,
            ) -> Result<types::TunnelGroupBackendList, Error> {
                self.c
                    .make_request("/backends/tunnel_group", reqwest::Method::GET, Some(req))
                    .await
            }

            /// List all TunnelGroup backends on this account
            pub fn list(&self, req: types::Paging) -> List {
                List {
                    c: std::sync::Arc::new(self.clone()),
                    req,
                }
            }

            /// Update TunnelGroup backend by ID
            pub async fn update(
                &self,
                req: &types::TunnelGroupBackendUpdate,
            ) -> Result<types::TunnelGroupBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/tunnel_group/{id}", id = req.id),
                        reqwest::Method::PATCH,
                        Some(req),
                    )
                    .await
            }
        }
    }

    /// A Weighted Backend balances traffic among the referenced backends. Traffic
    ///  is assigned proportionally to each based on its weight. The percentage of
    ///  traffic is calculated by dividing a backend's weight by the sum of all
    ///  weights.
    pub mod weighted {
        use crate::types;
        use crate::Error;
        use futures::{Stream, TryStreamExt};

        /// A Weighted Backend balances traffic among the referenced backends. Traffic
        ///  is assigned proportionally to each based on its weight. The percentage of
        ///  traffic is calculated by dividing a backend's weight by the sum of all
        ///  weights.
        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }
        /// Provides streams of pages of [types::WeightedBackendList], or of [types::WeightedBackend] directly.
        pub struct List {
            c: std::sync::Arc<Client>,
            req: types::Paging,
        }

        impl List {
            /// Iterate over [types::WeightedBackendList].
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn pages(
                self,
            ) -> impl Stream<Item = Result<types::WeightedBackendList, Error>> + Unpin {
                struct State {
                    c: std::sync::Arc<Client>,
                    req: types::Paging,
                    init: bool,
                    cur_uri: Option<String>,
                }

                let s = State {
                    c: self.c,
                    req: self.req,
                    init: true,
                    cur_uri: None,
                };

                Box::pin(futures::stream::unfold(s, |s| async move {
                    let page_resp = match (s.init, &s.cur_uri) {
                        (true, _) => s.c.list_page(&s.req).await,
                        (false, None) => {
                            return None;
                        }
                        (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

            /// Iterate over [types::WeightedBackend] items.
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn backends(
                self,
            ) -> impl Stream<Item = Result<types::WeightedBackend, Error>> + Unpin {
                self.pages()
                    .await
                    .map_ok(|page| futures::stream::iter(page.backends.into_iter().map(Ok)))
                    .try_flatten()
            }
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            /// Create a new Weighted backend
            pub async fn create(
                &self,
                req: &types::WeightedBackendCreate,
            ) -> Result<types::WeightedBackend, Error> {
                self.c
                    .make_request("/backends/weighted", reqwest::Method::POST, Some(req))
                    .await
            }

            /// Delete a Weighted backend by ID.
            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/backends/weighted/{id}", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get detailed information about a Weighted backend by ID
            pub async fn get(&self, id: &str) -> Result<types::WeightedBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/weighted/{id}", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get a single page without pagination. Prefer using list
            /// which will do pagination for you.
            pub(crate) async fn list_page(
                &self,
                req: &types::Paging,
            ) -> Result<types::WeightedBackendList, Error> {
                self.c
                    .make_request("/backends/weighted", reqwest::Method::GET, Some(req))
                    .await
            }

            /// List all Weighted backends on this account
            pub fn list(&self, req: types::Paging) -> List {
                List {
                    c: std::sync::Arc::new(self.clone()),
                    req,
                }
            }

            /// Update Weighted backend by ID
            pub async fn update(
                &self,
                req: &types::WeightedBackendUpdate,
            ) -> Result<types::WeightedBackend, Error> {
                self.c
                    .make_request(
                        &format!("/backends/weighted/{id}", id = req.id),
                        reqwest::Method::PATCH,
                        Some(req),
                    )
                    .await
            }
        }
    }
    impl Client {
        pub fn failover(&self) -> failover::Client {
            failover::Client::new(self.c.clone())
        }
        pub fn http_response(&self) -> http_response::Client {
            http_response::Client::new(self.c.clone())
        }
        pub fn static_address(&self) -> static_address::Client {
            static_address::Client::new(self.c.clone())
        }
        pub fn tunnel_group(&self) -> tunnel_group::Client {
            tunnel_group::Client::new(self.c.clone())
        }
        pub fn weighted(&self) -> weighted::Client {
            weighted::Client::new(self.c.clone())
        }
    }
}

pub mod edges {
    pub struct Client {
        c: crate::Client,
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }
    }

    pub mod https_routes {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            /// Create an HTTPS Edge Route
            pub async fn create(
                &self,
                req: &types::HTTPSEdgeRouteCreate,
            ) -> Result<types::HTTPSEdgeRoute, Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{edge_id}/routes", edge_id = req.edge_id),
                        reqwest::Method::POST,
                        Some(req),
                    )
                    .await
            }

            /// Get an HTTPS Edge Route by ID
            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::HTTPSEdgeRoute, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Updates an HTTPS Edge Route by ID. If a module is not specified in the update,
            /// it will not be modified. However, each module configuration that is specified
            /// will completely replace the existing value. There is no way to delete an
            /// existing module via this API, instead use the delete module API.
            pub async fn update(
                &self,
                req: &types::HTTPSEdgeRouteUpdate,
            ) -> Result<types::HTTPSEdgeRoute, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PATCH,
                        Some(req),
                    )
                    .await
            }

            /// Delete an HTTPS Edge Route by ID
            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https {
        use crate::types;
        use crate::Error;
        use futures::{Stream, TryStreamExt};

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }
        /// Provides streams of pages of [types::HTTPSEdgeList], or of [types::HTTPSEdge] directly.
        pub struct List {
            c: std::sync::Arc<Client>,
            req: types::Paging,
        }

        impl List {
            /// Iterate over [types::HTTPSEdgeList].
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn pages(
                self,
            ) -> impl Stream<Item = Result<types::HTTPSEdgeList, Error>> + Unpin {
                struct State {
                    c: std::sync::Arc<Client>,
                    req: types::Paging,
                    init: bool,
                    cur_uri: Option<String>,
                }

                let s = State {
                    c: self.c,
                    req: self.req,
                    init: true,
                    cur_uri: None,
                };

                Box::pin(futures::stream::unfold(s, |s| async move {
                    let page_resp = match (s.init, &s.cur_uri) {
                        (true, _) => s.c.list_page(&s.req).await,
                        (false, None) => {
                            return None;
                        }
                        (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

            /// Iterate over [types::HTTPSEdge] items.
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn https_edges(
                self,
            ) -> impl Stream<Item = Result<types::HTTPSEdge, Error>> + Unpin {
                self.pages()
                    .await
                    .map_ok(|page| futures::stream::iter(page.https_edges.into_iter().map(Ok)))
                    .try_flatten()
            }
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            /// Create an HTTPS Edge
            pub async fn create(
                &self,
                req: &types::HTTPSEdgeCreate,
            ) -> Result<types::HTTPSEdge, Error> {
                self.c
                    .make_request("/edges/https", reqwest::Method::POST, Some(req))
                    .await
            }

            /// Get an HTTPS Edge by ID
            pub async fn get(&self, id: &str) -> Result<types::HTTPSEdge, Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{id}", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get a single page without pagination. Prefer using list
            /// which will do pagination for you.
            pub(crate) async fn list_page(
                &self,
                req: &types::Paging,
            ) -> Result<types::HTTPSEdgeList, Error> {
                self.c
                    .make_request("/edges/https", reqwest::Method::GET, Some(req))
                    .await
            }

            /// Returns a list of all HTTPS Edges on this account
            pub fn list(&self, req: types::Paging) -> List {
                List {
                    c: std::sync::Arc::new(self.clone()),
                    req,
                }
            }

            /// Updates an HTTPS Edge by ID. If a module is not specified in the update, it will
            /// not be modified. However, each module configuration that is specified will
            /// completely replace the existing value. There is no way to delete an existing
            /// module via this API, instead use the delete module API.
            pub async fn update(
                &self,
                req: &types::HTTPSEdgeUpdate,
            ) -> Result<types::HTTPSEdge, Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{id}", id = req.id),
                        reqwest::Method::PATCH,
                        Some(req),
                    )
                    .await
            }

            /// Delete an HTTPS Edge by ID
            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{id}", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tcp {
        use crate::types;
        use crate::Error;
        use futures::{Stream, TryStreamExt};

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }
        /// Provides streams of pages of [types::TCPEdgeList], or of [types::TCPEdge] directly.
        pub struct List {
            c: std::sync::Arc<Client>,
            req: types::Paging,
        }

        impl List {
            /// Iterate over [types::TCPEdgeList].
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn pages(
                self,
            ) -> impl Stream<Item = Result<types::TCPEdgeList, Error>> + Unpin {
                struct State {
                    c: std::sync::Arc<Client>,
                    req: types::Paging,
                    init: bool,
                    cur_uri: Option<String>,
                }

                let s = State {
                    c: self.c,
                    req: self.req,
                    init: true,
                    cur_uri: None,
                };

                Box::pin(futures::stream::unfold(s, |s| async move {
                    let page_resp = match (s.init, &s.cur_uri) {
                        (true, _) => s.c.list_page(&s.req).await,
                        (false, None) => {
                            return None;
                        }
                        (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

            /// Iterate over [types::TCPEdge] items.
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn tcp_edges(
                self,
            ) -> impl Stream<Item = Result<types::TCPEdge, Error>> + Unpin {
                self.pages()
                    .await
                    .map_ok(|page| futures::stream::iter(page.tcp_edges.into_iter().map(Ok)))
                    .try_flatten()
            }
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            /// Create a TCP Edge
            pub async fn create(
                &self,
                req: &types::TCPEdgeCreate,
            ) -> Result<types::TCPEdge, Error> {
                self.c
                    .make_request("/edges/tcp", reqwest::Method::POST, Some(req))
                    .await
            }

            /// Get a TCP Edge by ID
            pub async fn get(&self, id: &str) -> Result<types::TCPEdge, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get a single page without pagination. Prefer using list
            /// which will do pagination for you.
            pub(crate) async fn list_page(
                &self,
                req: &types::Paging,
            ) -> Result<types::TCPEdgeList, Error> {
                self.c
                    .make_request("/edges/tcp", reqwest::Method::GET, Some(req))
                    .await
            }

            /// Returns a list of all TCP Edges on this account
            pub fn list(&self, req: types::Paging) -> List {
                List {
                    c: std::sync::Arc::new(self.clone()),
                    req,
                }
            }

            /// Updates a TCP Edge by ID. If a module is not specified in the update, it will
            /// not be modified. However, each module configuration that is specified will
            /// completely replace the existing value. There is no way to delete an existing
            /// module via this API, instead use the delete module API.
            pub async fn update(
                &self,
                req: &types::TCPEdgeUpdate,
            ) -> Result<types::TCPEdge, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}", id = req.id),
                        reqwest::Method::PATCH,
                        Some(req),
                    )
                    .await
            }

            /// Delete a TCP Edge by ID
            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tls {
        use crate::types;
        use crate::Error;
        use futures::{Stream, TryStreamExt};

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }
        /// Provides streams of pages of [types::TLSEdgeList], or of [types::TLSEdge] directly.
        pub struct List {
            c: std::sync::Arc<Client>,
            req: types::Paging,
        }

        impl List {
            /// Iterate over [types::TLSEdgeList].
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn pages(
                self,
            ) -> impl Stream<Item = Result<types::TLSEdgeList, Error>> + Unpin {
                struct State {
                    c: std::sync::Arc<Client>,
                    req: types::Paging,
                    init: bool,
                    cur_uri: Option<String>,
                }

                let s = State {
                    c: self.c,
                    req: self.req,
                    init: true,
                    cur_uri: None,
                };

                Box::pin(futures::stream::unfold(s, |s| async move {
                    let page_resp = match (s.init, &s.cur_uri) {
                        (true, _) => s.c.list_page(&s.req).await,
                        (false, None) => {
                            return None;
                        }
                        (false, Some(uri)) => s.c.c.get_by_uri(uri).await,
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

            /// Iterate over [types::TLSEdge] items.
            ///
            /// See [Tokio Streams](https://tokio.rs/tokio/tutorial/streams)
            /// documentation for more info on usage.
            pub async fn tls_edges(
                self,
            ) -> impl Stream<Item = Result<types::TLSEdge, Error>> + Unpin {
                self.pages()
                    .await
                    .map_ok(|page| futures::stream::iter(page.tls_edges.into_iter().map(Ok)))
                    .try_flatten()
            }
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            /// Create a TLS Edge
            pub async fn create(
                &self,
                req: &types::TLSEdgeCreate,
            ) -> Result<types::TLSEdge, Error> {
                self.c
                    .make_request("/edges/tls", reqwest::Method::POST, Some(req))
                    .await
            }

            /// Get a TLS Edge by ID
            pub async fn get(&self, id: &str) -> Result<types::TLSEdge, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            /// Get a single page without pagination. Prefer using list
            /// which will do pagination for you.
            pub(crate) async fn list_page(
                &self,
                req: &types::Paging,
            ) -> Result<types::TLSEdgeList, Error> {
                self.c
                    .make_request("/edges/tls", reqwest::Method::GET, Some(req))
                    .await
            }

            /// Returns a list of all TLS Edges on this account
            pub fn list(&self, req: types::Paging) -> List {
                List {
                    c: std::sync::Arc::new(self.clone()),
                    req,
                }
            }

            /// Updates a TLS Edge by ID. If a module is not specified in the update, it will
            /// not be modified. However, each module configuration that is specified will
            /// completely replace the existing value. There is no way to delete an existing
            /// module via this API, instead use the delete module API.
            pub async fn update(
                &self,
                req: &types::TLSEdgeUpdate,
            ) -> Result<types::TLSEdge, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}", id = req.id),
                        reqwest::Method::PATCH,
                        Some(req),
                    )
                    .await
            }

            /// Delete a TLS Edge by ID
            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }
    impl Client {
        pub fn https_routes(&self) -> https_routes::Client {
            https_routes::Client::new(self.c.clone())
        }
        pub fn https(&self) -> https::Client {
            https::Client::new(self.c.clone())
        }
        pub fn tcp(&self) -> tcp::Client {
            tcp::Client::new(self.c.clone())
        }
        pub fn tls(&self) -> tls::Client {
            tls::Client::new(self.c.clone())
        }
    }
}

pub mod edge_modules {
    pub struct Client {
        c: crate::Client,
    }

    impl Client {
        pub fn new(c: crate::Client) -> Self {
            Self { c }
        }
    }

    pub mod https_edge_mutual_tls {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeMutualTLSReplace,
            ) -> Result<types::EndpointMutualTLS, Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{id}/mutual_tls", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointMutualTLS, Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{id}/mutual_tls", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{id}/mutual_tls", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_tls_termination {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeTLSTerminationAtEdgeReplace,
            ) -> Result<types::EndpointTLSTermination, Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{id}/tls_termination", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointTLSTermination, Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{id}/tls_termination", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/https/{id}/tls_termination", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_backend {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteBackendReplace,
            ) -> Result<types::EndpointBackend, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/backend",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointBackend, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/backend",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/backend",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_ip_restriction {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteIPRestrictionReplace,
            ) -> Result<types::EndpointIPPolicy, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/ip_restriction",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointIPPolicy, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/ip_restriction",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/ip_restriction",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_request_headers {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteRequestHeadersReplace,
            ) -> Result<types::EndpointRequestHeaders, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/request_headers",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointRequestHeaders, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/request_headers",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/request_headers",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_response_headers {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteResponseHeadersReplace,
            ) -> Result<types::EndpointResponseHeaders, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/response_headers",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointResponseHeaders, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/response_headers",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/response_headers",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_compression {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteCompressionReplace,
            ) -> Result<types::EndpointCompression, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/compression",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointCompression, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/compression",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/compression",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_circuit_breaker {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteCircuitBreakerReplace,
            ) -> Result<types::EndpointCircuitBreaker, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/circuit_breaker",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointCircuitBreaker, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/circuit_breaker",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/circuit_breaker",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_webhook_verification {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteWebhookVerificationReplace,
            ) -> Result<types::EndpointWebhookValidation, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/webhook_verification",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointWebhookValidation, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/webhook_verification",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/webhook_verification",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_oauth {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteOAuthReplace,
            ) -> Result<types::EndpointOAuth, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/oauth",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointOAuth, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/oauth",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/oauth",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_saml {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteSAMLReplace,
            ) -> Result<types::EndpointSAML, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/saml",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointSAML, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/saml",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/saml",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_oidc {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteOIDCReplace,
            ) -> Result<types::EndpointOIDC, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/oidc",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointOIDC, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/oidc",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/oidc",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_websocket_tcp_converter {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteWebsocketTCPConverterReplace,
            ) -> Result<types::EndpointWebsocketTCPConverter, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/websocket_tcp_converter",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointWebsocketTCPConverter, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/websocket_tcp_converter",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/websocket_tcp_converter",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_user_agent_filter {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteUserAgentFilterReplace,
            ) -> Result<types::EndpointUserAgentFilter, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/user_agent_filter",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointUserAgentFilter, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/user_agent_filter",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/user_agent_filter",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod https_edge_route_traffic_policy {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeRouteTrafficPolicyReplace,
            ) -> Result<types::EndpointTrafficPolicy, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/traffic_policy",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(
                &self,
                req: &types::EdgeRouteItem,
            ) -> Result<types::EndpointTrafficPolicy, Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/traffic_policy",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, req: &types::EdgeRouteItem) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!(
                            "/edges/https/{edge_id}/routes/{id}/traffic_policy",
                            edge_id = req.edge_id,
                            id = req.id
                        ),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tcp_edge_backend {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeBackendReplace,
            ) -> Result<types::EndpointBackend, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}/backend", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointBackend, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}/backend", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}/backend", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tcp_edge_ip_restriction {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeIPRestrictionReplace,
            ) -> Result<types::EndpointIPPolicy, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}/ip_restriction", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointIPPolicy, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}/ip_restriction", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}/ip_restriction", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tcp_edge_traffic_policy {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeTrafficPolicyReplace,
            ) -> Result<types::EndpointTrafficPolicy, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}/traffic_policy", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointTrafficPolicy, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}/traffic_policy", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tcp/{id}/traffic_policy", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tls_edge_backend {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeBackendReplace,
            ) -> Result<types::EndpointBackend, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/backend", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointBackend, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/backend", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/backend", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tls_edge_ip_restriction {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeIPRestrictionReplace,
            ) -> Result<types::EndpointIPPolicy, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/ip_restriction", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointIPPolicy, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/ip_restriction", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/ip_restriction", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tls_edge_mutual_tls {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeMutualTLSReplace,
            ) -> Result<types::EndpointMutualTLS, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/mutual_tls", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointMutualTLS, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/mutual_tls", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/mutual_tls", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tls_edge_tls_termination {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeTLSTerminationReplace,
            ) -> Result<types::EndpointTLSTermination, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/tls_termination", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointTLSTermination, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/tls_termination", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/tls_termination", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }

    pub mod tls_edge_traffic_policy {
        use crate::types;
        use crate::Error;

        #[derive(Debug, Clone)]
        pub struct Client {
            c: crate::Client,
        }

        impl Client {
            pub fn new(c: crate::Client) -> Self {
                Self { c }
            }

            pub async fn replace(
                &self,
                req: &types::EdgeTrafficPolicyReplace,
            ) -> Result<types::EndpointTrafficPolicy, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/traffic_policy", id = req.id),
                        reqwest::Method::PUT,
                        Some(req),
                    )
                    .await
            }

            pub async fn get(&self, id: &str) -> Result<types::EndpointTrafficPolicy, Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/traffic_policy", id = id),
                        reqwest::Method::GET,
                        None::<Option<()>>,
                    )
                    .await
            }

            pub async fn delete(&self, id: &str) -> Result<(), Error> {
                self.c
                    .make_request(
                        &format!("/edges/tls/{id}/traffic_policy", id = id),
                        reqwest::Method::DELETE,
                        None::<Option<()>>,
                    )
                    .await
            }
        }
    }
    impl Client {
        pub fn https_edge_mutual_tls(&self) -> https_edge_mutual_tls::Client {
            https_edge_mutual_tls::Client::new(self.c.clone())
        }
        pub fn https_edge_tls_termination(&self) -> https_edge_tls_termination::Client {
            https_edge_tls_termination::Client::new(self.c.clone())
        }
        pub fn https_edge_route_backend(&self) -> https_edge_route_backend::Client {
            https_edge_route_backend::Client::new(self.c.clone())
        }
        pub fn https_edge_route_ip_restriction(&self) -> https_edge_route_ip_restriction::Client {
            https_edge_route_ip_restriction::Client::new(self.c.clone())
        }
        pub fn https_edge_route_request_headers(&self) -> https_edge_route_request_headers::Client {
            https_edge_route_request_headers::Client::new(self.c.clone())
        }
        pub fn https_edge_route_response_headers(
            &self,
        ) -> https_edge_route_response_headers::Client {
            https_edge_route_response_headers::Client::new(self.c.clone())
        }
        pub fn https_edge_route_compression(&self) -> https_edge_route_compression::Client {
            https_edge_route_compression::Client::new(self.c.clone())
        }
        pub fn https_edge_route_circuit_breaker(&self) -> https_edge_route_circuit_breaker::Client {
            https_edge_route_circuit_breaker::Client::new(self.c.clone())
        }
        pub fn https_edge_route_webhook_verification(
            &self,
        ) -> https_edge_route_webhook_verification::Client {
            https_edge_route_webhook_verification::Client::new(self.c.clone())
        }
        pub fn https_edge_route_oauth(&self) -> https_edge_route_oauth::Client {
            https_edge_route_oauth::Client::new(self.c.clone())
        }
        pub fn https_edge_route_saml(&self) -> https_edge_route_saml::Client {
            https_edge_route_saml::Client::new(self.c.clone())
        }
        pub fn https_edge_route_oidc(&self) -> https_edge_route_oidc::Client {
            https_edge_route_oidc::Client::new(self.c.clone())
        }
        pub fn https_edge_route_websocket_tcp_converter(
            &self,
        ) -> https_edge_route_websocket_tcp_converter::Client {
            https_edge_route_websocket_tcp_converter::Client::new(self.c.clone())
        }
        pub fn https_edge_route_user_agent_filter(
            &self,
        ) -> https_edge_route_user_agent_filter::Client {
            https_edge_route_user_agent_filter::Client::new(self.c.clone())
        }
        pub fn https_edge_route_traffic_policy(&self) -> https_edge_route_traffic_policy::Client {
            https_edge_route_traffic_policy::Client::new(self.c.clone())
        }
        pub fn tcp_edge_backend(&self) -> tcp_edge_backend::Client {
            tcp_edge_backend::Client::new(self.c.clone())
        }
        pub fn tcp_edge_ip_restriction(&self) -> tcp_edge_ip_restriction::Client {
            tcp_edge_ip_restriction::Client::new(self.c.clone())
        }
        pub fn tcp_edge_traffic_policy(&self) -> tcp_edge_traffic_policy::Client {
            tcp_edge_traffic_policy::Client::new(self.c.clone())
        }
        pub fn tls_edge_backend(&self) -> tls_edge_backend::Client {
            tls_edge_backend::Client::new(self.c.clone())
        }
        pub fn tls_edge_ip_restriction(&self) -> tls_edge_ip_restriction::Client {
            tls_edge_ip_restriction::Client::new(self.c.clone())
        }
        pub fn tls_edge_mutual_tls(&self) -> tls_edge_mutual_tls::Client {
            tls_edge_mutual_tls::Client::new(self.c.clone())
        }
        pub fn tls_edge_tls_termination(&self) -> tls_edge_tls_termination::Client {
            tls_edge_tls_termination::Client::new(self.c.clone())
        }
        pub fn tls_edge_traffic_policy(&self) -> tls_edge_traffic_policy::Client {
            tls_edge_traffic_policy::Client::new(self.c.clone())
        }
    }
}

impl Client {
    pub fn backends(&self) -> backends::Client {
        backends::Client::new(self.clone())
    }
    pub fn edges(&self) -> edges::Client {
        edges::Client::new(self.clone())
    }
    pub fn edge_modules(&self) -> edge_modules::Client {
        edge_modules::Client::new(self.clone())
    }
}
