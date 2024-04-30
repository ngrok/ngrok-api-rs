// Code generated for API Clients. DO NOT EDIT.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Item {
    /// a resource identifier
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Paging {
    pub before_id: Option<String>,
    pub limit: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Error {
    pub error_code: String,
    pub status_code: i32,
    pub msg: String,
    pub details: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Ref {
    /// a resource identifier
    pub id: String,
    /// a uri for locating a resource
    pub uri: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AbuseReport {
    /// ID of the abuse report
    pub id: String,
    /// URI of the abuse report API resource
    pub uri: String,
    /// timestamp that the abuse report record was created in RFC 3339 format
    pub created_at: String,
    /// a list of URLs containing suspected abusive content
    pub urls: Vec<String>,
    /// arbitrary user-defined data about this abuse report. Optional, max 4096 bytes.
    pub metadata: String,
    /// Indicates whether ngrok has processed the abuse report. one of `PENDING`,
    /// `PROCESSED`, or `PARTIALLY_PROCESSED`
    pub status: String,
    /// an array of hostname statuses related to the report
    pub hostnames: Vec<AbuseReportHostname>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AbuseReportHostname {
    /// the hostname ngrok has parsed out of one of the reported URLs in this abuse
    /// report
    pub hostname: String,
    /// indicates what action ngrok has taken against the hostname. one of `PENDING`,
    /// `BANNED`, `UNBANNED`, or `IGNORE`
    pub status: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AbuseReportCreate {
    /// a list of URLs containing suspected abusive content
    pub urls: Vec<String>,
    /// arbitrary user-defined data about this abuse report. Optional, max 4096 bytes.
    pub metadata: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AgentIngressCreate {
    /// human-readable description of the use of this Agent Ingress. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this Agent Ingress. optional,
    /// max 4096 bytes
    pub metadata: String,
    /// the domain that you own to be used as the base domain name to generate regional
    /// agent ingress domains.
    pub domain: String,
    /// configuration for automatic management of TLS certificates for this domain, or
    /// null if automatic management is disabled. Optional.
    pub certificate_management_policy: Option<AgentIngressCertPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AgentIngressUpdate {
    pub id: String,
    /// human-readable description of the use of this Agent Ingress. optional, max 255
    /// bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this Agent Ingress. optional,
    /// max 4096 bytes
    pub metadata: Option<String>,
    /// configuration for automatic management of TLS certificates for this domain, or
    /// null if automatic management is disabled. Optional.
    pub certificate_management_policy: Option<AgentIngressCertPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AgentIngress {
    /// unique Agent Ingress resource identifier
    pub id: String,
    /// URI to the API resource of this Agent ingress
    pub uri: String,
    /// human-readable description of the use of this Agent Ingress. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this Agent Ingress. optional,
    /// max 4096 bytes
    pub metadata: String,
    /// the domain that you own to be used as the base domain name to generate regional
    /// agent ingress domains.
    pub domain: String,
    /// a list of target values to use as the values of NS records for the domain
    /// property these values will delegate control over the domain to ngrok
    pub ns_targets: Vec<String>,
    /// a list of regional agent ingress domains that are subdomains of the value of
    /// domain this value may increase over time as ngrok adds more regions
    pub region_domains: Vec<String>,
    /// timestamp when the Agent Ingress was created, RFC 3339 format
    pub created_at: String,
    /// configuration for automatic management of TLS certificates for this domain, or
    /// null if automatic management is disabled
    pub certificate_management_policy: Option<AgentIngressCertPolicy>,
    /// status of the automatic certificate management for this domain, or null if
    /// automatic management is disabled
    pub certificate_management_status: Option<AgentIngressCertStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AgentIngressList {
    /// the list of Agent Ingresses owned by this account
    pub ingresses: Vec<AgentIngress>,
    /// URI of the Agent Ingress list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AgentIngressCertPolicy {
    /// certificate authority to request certificates from. The only supported value is
    /// letsencrypt.
    pub authority: String,
    /// type of private key to use when requesting certificates. Defaults to rsa, can be
    /// either rsa or ecdsa.
    pub private_key_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AgentIngressCertStatus {
    /// timestamp when the next renewal will be requested, RFC 3339 format
    pub renews_at: Option<String>,
    /// status of the certificate provisioning job, or null if the certificiate isn't
    /// being provisioned or renewed
    pub provisioning_job: Option<AgentIngressCertJob>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AgentIngressCertJob {
    /// if present, an error code indicating why provisioning is failing. It may be
    /// either a temporary condition (INTERNAL_ERROR), or a permanent one the user must
    /// correct (DNS_ERROR).
    pub error_code: Option<String>,
    /// a message describing the current status or error
    pub msg: String,
    /// timestamp when the provisioning job started, RFC 3339 format
    pub started_at: String,
    /// timestamp when the provisioning job will be retried
    pub retries_at: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct APIKeyCreate {
    /// human-readable description of what uses the API key to authenticate. optional,
    /// max 255 bytes.
    pub description: String,
    /// arbitrary user-defined data of this API key. optional, max 4096 bytes
    pub metadata: String,
    /// If supplied at credential creation, ownership will be assigned to the specified
    /// User or Bot. Only admins may specify an owner other than themselves. Defaults to
    /// the authenticated User or Bot.
    pub owner_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct APIKeyUpdate {
    pub id: String,
    /// human-readable description of what uses the API key to authenticate. optional,
    /// max 255 bytes.
    pub description: Option<String>,
    /// arbitrary user-defined data of this API key. optional, max 4096 bytes
    pub metadata: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct APIKey {
    /// unique API key resource identifier
    pub id: String,
    /// URI to the API resource of this API key
    pub uri: String,
    /// human-readable description of what uses the API key to authenticate. optional,
    /// max 255 bytes.
    pub description: String,
    /// arbitrary user-defined data of this API key. optional, max 4096 bytes
    pub metadata: String,
    /// timestamp when the api key was created, RFC 3339 format
    pub created_at: String,
    /// the bearer token that can be placed into the Authorization header to
    /// authenticate request to the ngrok API. **This value is only available one time,
    /// on the API response from key creation. Otherwise it is null.**
    pub token: Option<String>,
    /// If supplied at credential creation, ownership will be assigned to the specified
    /// User or Bot. Only admins may specify an owner other than themselves. Defaults to
    /// the authenticated User or Bot.
    pub owner_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct APIKeyList {
    /// the list of API keys for this account
    pub keys: Vec<APIKey>,
    /// URI of the API keys list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplicationSession {
    /// unique application session resource identifier
    pub id: String,
    /// URI of the application session API resource
    pub uri: String,
    /// URL of the hostport served by this endpoint
    pub public_url: String,
    /// browser session details of the application session
    pub browser_session: BrowserSession,
    /// application user this session is associated with
    pub application_user: Option<Ref>,
    /// timestamp when the user was created in RFC 3339 format
    pub created_at: String,
    /// timestamp when the user was last active in RFC 3339 format
    pub last_active: String,
    /// timestamp when session expires in RFC 3339 format
    pub expires_at: String,
    /// ephemeral endpoint this session is associated with
    pub endpoint: Option<Ref>,
    /// edge this session is associated with, null if the endpoint is agent-initiated
    pub edge: Option<Ref>,
    /// route this session is associated with, null if the endpoint is agent-initiated
    pub route: Option<Ref>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplicationSessionList {
    /// list of all application sessions on this account
    pub application_sessions: Vec<ApplicationSession>,
    /// URI of the application session list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BrowserSession {
    /// HTTP User-Agent data
    pub user_agent: UserAgent,
    /// IP address
    pub ip_address: String,
    /// IP geolocation data
    pub location: Option<Location>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserAgent {
    /// raw User-Agent request header
    pub raw: String,
    /// browser name (e.g. Chrome)
    pub browser_name: String,
    /// browser version (e.g. 102)
    pub browser_version: String,
    /// type of device (e.g. Desktop)
    pub device_type: String,
    /// operating system name (e.g. MacOS)
    pub os_name: String,
    /// operating system version (e.g. 10.15.7)
    pub os_version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Location {
    /// ISO country code
    pub country_code: Option<String>,
    /// geographical latitude
    pub latitude: Option<f64>,
    /// geographical longitude
    pub longitude: Option<f64>,
    /// accuracy radius of the geographical coordinates
    pub lat_long_radius_km: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplicationUser {
    /// unique application user resource identifier
    pub id: String,
    /// URI of the application user API resource
    pub uri: String,
    /// identity provider that the user authenticated with
    pub identity_provider: IdentityProvider,
    /// unique user identifier
    pub provider_user_id: String,
    /// user username
    pub username: String,
    /// user email
    pub email: String,
    /// user common name
    pub name: String,
    /// timestamp when the user was created in RFC 3339 format
    pub created_at: String,
    /// timestamp when the user was last active in RFC 3339 format
    pub last_active: String,
    /// timestamp when the user last signed-in in RFC 3339 format
    pub last_login: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplicationUserList {
    /// list of all application users on this account
    pub application_users: Vec<ApplicationUser>,
    /// URI of the application user list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IdentityProvider {
    /// name of the identity provider (e.g. Google)
    pub name: String,
    /// URL of the identity provider (e.g. https://accounts.google.com)
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TunnelSession {
    /// version of the ngrok agent that started this ngrok tunnel session
    pub agent_version: String,
    /// reference to the tunnel credential or ssh credential used by the ngrok agent to
    /// start this tunnel session
    pub credential: Ref,
    /// unique tunnel session resource identifier
    pub id: String,
    /// source ip address of the tunnel session
    pub ip: String,
    /// arbitrary user-defined data specified in the metadata property in the ngrok
    /// configuration file. See the metadata configuration option
    pub metadata: String,
    /// operating system of the host the ngrok agent is running on
    pub os: String,
    /// the ngrok region identifier in which this tunnel session was started
    pub region: String,
    /// time when the tunnel session first connected to the ngrok servers
    pub started_at: String,
    /// the transport protocol used to start the tunnel session. Either `ngrok/v2` or
    /// `ssh`
    pub transport: String,
    /// URI to the API resource of the tunnel session
    pub uri: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TunnelSessionList {
    /// list of all tunnel sessions on this account
    pub tunnel_sessions: Vec<TunnelSession>,
    /// URI to the API resource of the tunnel session list
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TunnelSessionsUpdate {
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FailoverBackend {
    /// unique identifier for this Failover backend
    pub id: String,
    /// URI of the FailoverBackend API resource
    pub uri: String,
    /// timestamp when the backend was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// the ids of the child backends in order
    pub backends: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FailoverBackendCreate {
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// the ids of the child backends in order
    pub backends: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FailoverBackendUpdate {
    pub id: String,
    /// human-readable description of this backend. Optional
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: Option<String>,
    /// the ids of the child backends in order
    pub backends: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FailoverBackendList {
    /// the list of all Failover backends on this account
    pub backends: Vec<FailoverBackend>,
    /// URI of the Failover backends list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPResponseBackend {
    pub id: String,
    /// URI of the HTTPResponseBackend API resource
    pub uri: String,
    /// timestamp when the backend was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// body to return as fixed content
    pub body: String,
    /// headers to return
    pub headers: HashMap<String, String>,
    /// status code to return
    pub status_code: i32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPResponseBackendCreate {
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// body to return as fixed content
    pub body: String,
    /// headers to return
    pub headers: HashMap<String, String>,
    /// status code to return
    pub status_code: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPResponseBackendUpdate {
    pub id: String,
    /// human-readable description of this backend. Optional
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: Option<String>,
    /// body to return as fixed content
    pub body: Option<String>,
    /// headers to return
    pub headers: Option<HashMap<String, String>>,
    /// status code to return
    pub status_code: Option<i32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPResponseBackendList {
    pub backends: Vec<HTTPResponseBackend>,
    pub uri: String,
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StaticBackend {
    /// unique identifier for this static backend
    pub id: String,
    /// URI of the StaticBackend API resource
    pub uri: String,
    /// timestamp when the backend was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// the address to forward to
    pub address: String,
    /// tls configuration to use
    pub tls: StaticBackendTLS,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StaticBackendTLS {
    /// if TLS is checked
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StaticBackendCreate {
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// the address to forward to
    pub address: String,
    /// tls configuration to use
    pub tls: StaticBackendTLS,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StaticBackendUpdate {
    pub id: String,
    /// human-readable description of this backend. Optional
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: Option<String>,
    /// the address to forward to
    pub address: String,
    /// tls configuration to use
    pub tls: StaticBackendTLS,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StaticBackendList {
    /// the list of all static backends on this account
    pub backends: Vec<StaticBackend>,
    /// URI of the static backends list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TunnelGroupBackend {
    /// unique identifier for this TunnelGroup backend
    pub id: String,
    /// URI of the TunnelGroupBackend API resource
    pub uri: String,
    /// timestamp when the backend was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// labels to watch for tunnels on, e.g. app->foo, dc->bar
    pub labels: HashMap<String, String>,
    /// tunnels matching this backend
    pub tunnels: Vec<Ref>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TunnelGroupBackendCreate {
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// labels to watch for tunnels on, e.g. app->foo, dc->bar
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TunnelGroupBackendUpdate {
    pub id: String,
    /// human-readable description of this backend. Optional
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: Option<String>,
    /// labels to watch for tunnels on, e.g. app->foo, dc->bar
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TunnelGroupBackendList {
    /// the list of all TunnelGroup backends on this account
    pub backends: Vec<TunnelGroupBackend>,
    /// URI of the TunnelGroup backends list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WeightedBackend {
    /// unique identifier for this Weighted backend
    pub id: String,
    /// URI of the WeightedBackend API resource
    pub uri: String,
    /// timestamp when the backend was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// the ids of the child backends to their weights [0-10000]
    pub backends: HashMap<String, i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WeightedBackendCreate {
    /// human-readable description of this backend. Optional
    pub description: String,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: String,
    /// the ids of the child backends to their weights [0-10000]
    pub backends: HashMap<String, i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WeightedBackendUpdate {
    pub id: String,
    /// human-readable description of this backend. Optional
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this backend. Optional
    pub metadata: Option<String>,
    /// the ids of the child backends to their weights [0-10000]
    pub backends: HashMap<String, i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WeightedBackendList {
    /// the list of all Weighted backends on this account
    pub backends: Vec<WeightedBackend>,
    /// URI of the Weighted backends list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BotUser {
    /// unique API key resource identifier
    pub id: String,
    /// URI to the API resource of this bot user
    pub uri: String,
    /// human-readable name used to identify the bot
    pub name: String,
    /// whether or not the bot is active
    pub active: bool,
    /// timestamp when the api key was created, RFC 3339 format
    pub created_at: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BotUserCreate {
    /// human-readable name used to identify the bot
    pub name: String,
    /// whether or not the bot is active
    pub active: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BotUserUpdate {
    pub id: String,
    /// human-readable name used to identify the bot
    pub name: Option<String>,
    /// whether or not the bot is active
    pub active: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BotUserList {
    /// the list of all bot users on this account
    pub bot_users: Vec<BotUser>,
    /// URI of the bot users list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CertificateAuthorityCreate {
    /// human-readable description of this Certificate Authority. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this Certificate Authority.
    /// optional, max 4096 bytes.
    pub metadata: String,
    /// raw PEM of the Certificate Authority
    pub ca_pem: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CertificateAuthorityUpdate {
    pub id: String,
    /// human-readable description of this Certificate Authority. optional, max 255
    /// bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this Certificate Authority.
    /// optional, max 4096 bytes.
    pub metadata: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CertificateAuthority {
    /// unique identifier for this Certificate Authority
    pub id: String,
    /// URI of the Certificate Authority API resource
    pub uri: String,
    /// timestamp when the Certificate Authority was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of this Certificate Authority. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this Certificate Authority.
    /// optional, max 4096 bytes.
    pub metadata: String,
    /// raw PEM of the Certificate Authority
    pub ca_pem: String,
    /// subject common name of the Certificate Authority
    pub subject_common_name: String,
    /// timestamp when this Certificate Authority becomes valid, RFC 3339 format
    pub not_before: String,
    /// timestamp when this Certificate Authority becomes invalid, RFC 3339 format
    pub not_after: String,
    /// set of actions the private key of this Certificate Authority can be used for
    pub key_usages: Vec<String>,
    /// extended set of actions the private key of this Certificate Authority can be
    /// used for
    pub extended_key_usages: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CertificateAuthorityList {
    /// the list of all certificate authorities on this account
    pub certificate_authorities: Vec<CertificateAuthority>,
    /// URI of the certificates authorities list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CredentialCreate {
    /// human-readable description of who or what will use the credential to
    /// authenticate. Optional, max 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this credential. Optional, max
    /// 4096 bytes.
    pub metadata: String,
    /// optional list of ACL rules. If unspecified, the credential will have no
    /// restrictions. The only allowed ACL rule at this time is the `bind` rule. The
    /// `bind` rule allows the caller to restrict what domains, addresses, and labels
    /// the token is allowed to bind. For example, to allow the token to open a tunnel
    /// on example.ngrok.io your ACL would include the rule `bind:example.ngrok.io`.
    /// Bind rules for domains may specify a leading wildcard to match multiple domains
    /// with a common suffix. For example, you may specify a rule of
    /// `bind:*.example.com` which will allow `x.example.com`, `y.example.com`,
    /// `*.example.com`, etc. Bind rules for labels may specify a wildcard key and/or
    /// value to match multiple labels. For example, you may specify a rule of
    /// `bind:*=example` which will allow `x=example`, `y=example`, etc. A rule of `'*'`
    /// is equivalent to no acl at all and will explicitly permit all actions.
    pub acl: Vec<String>,
    /// If supplied at credential creation, ownership will be assigned to the specified
    /// User or Bot. Only admins may specify an owner other than themselves. Defaults to
    /// the authenticated User or Bot.
    pub owner_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CredentialUpdate {
    pub id: String,
    /// human-readable description of who or what will use the credential to
    /// authenticate. Optional, max 255 bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this credential. Optional, max
    /// 4096 bytes.
    pub metadata: Option<String>,
    /// optional list of ACL rules. If unspecified, the credential will have no
    /// restrictions. The only allowed ACL rule at this time is the `bind` rule. The
    /// `bind` rule allows the caller to restrict what domains, addresses, and labels
    /// the token is allowed to bind. For example, to allow the token to open a tunnel
    /// on example.ngrok.io your ACL would include the rule `bind:example.ngrok.io`.
    /// Bind rules for domains may specify a leading wildcard to match multiple domains
    /// with a common suffix. For example, you may specify a rule of
    /// `bind:*.example.com` which will allow `x.example.com`, `y.example.com`,
    /// `*.example.com`, etc. Bind rules for labels may specify a wildcard key and/or
    /// value to match multiple labels. For example, you may specify a rule of
    /// `bind:*=example` which will allow `x=example`, `y=example`, etc. A rule of `'*'`
    /// is equivalent to no acl at all and will explicitly permit all actions.
    pub acl: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Credential {
    /// unique tunnel credential resource identifier
    pub id: String,
    /// URI of the tunnel credential API resource
    pub uri: String,
    /// timestamp when the tunnel credential was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of who or what will use the credential to
    /// authenticate. Optional, max 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this credential. Optional, max
    /// 4096 bytes.
    pub metadata: String,
    /// the credential's authtoken that can be used to authenticate an ngrok agent.
    /// **This value is only available one time, on the API response from credential
    /// creation, otherwise it is null.**
    pub token: Option<String>,
    /// optional list of ACL rules. If unspecified, the credential will have no
    /// restrictions. The only allowed ACL rule at this time is the `bind` rule. The
    /// `bind` rule allows the caller to restrict what domains, addresses, and labels
    /// the token is allowed to bind. For example, to allow the token to open a tunnel
    /// on example.ngrok.io your ACL would include the rule `bind:example.ngrok.io`.
    /// Bind rules for domains may specify a leading wildcard to match multiple domains
    /// with a common suffix. For example, you may specify a rule of
    /// `bind:*.example.com` which will allow `x.example.com`, `y.example.com`,
    /// `*.example.com`, etc. Bind rules for labels may specify a wildcard key and/or
    /// value to match multiple labels. For example, you may specify a rule of
    /// `bind:*=example` which will allow `x=example`, `y=example`, etc. A rule of `'*'`
    /// is equivalent to no acl at all and will explicitly permit all actions.
    pub acl: Vec<String>,
    /// If supplied at credential creation, ownership will be assigned to the specified
    /// User or Bot. Only admins may specify an owner other than themselves. Defaults to
    /// the authenticated User or Bot.
    pub owner_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CredentialList {
    /// the list of all tunnel credentials on this account
    pub credentials: Vec<Credential>,
    /// URI of the tunnel credential list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointWebhookValidation {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// a string indicating which webhook provider will be sending webhooks to this
    /// endpoint. Value must be one of the supported providers defined at
    /// https://ngrok.com/docs/cloud-edge/modules/webhook-verification
    pub provider: String,
    /// a string secret used to validate requests from the given provider. All providers
    /// except AWS SNS require a secret
    pub secret: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointCompression {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointMutualTLS {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// PEM-encoded CA certificates that will be used to validate. Multiple CAs may be
    /// provided by concatenating them together.
    pub certificate_authorities: Vec<Ref>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointMutualTLSMutate {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// list of certificate authorities that will be used to validate the TLS client
    /// certificate presented by the initiator of the TLS connection
    pub certificate_authority_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointTLSTermination {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// `edge` if the ngrok edge should terminate TLS traffic, `upstream` if TLS traffic
    /// should be passed through to the upstream ngrok agent / application server for
    /// termination. if `upstream` is chosen, most other modules will be disallowed
    /// because they rely on the ngrok edge being able to access the underlying traffic.
    pub terminate_at: String,
    /// The minimum TLS version used for termination and advertised to the client during
    /// the TLS handshake. if unspecified, ngrok will choose an industry-safe default.
    /// This value must be null if `terminate_at` is set to `upstream`.
    pub min_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointTLSTerminationAtEdge {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// The minimum TLS version used for termination and advertised to the client during
    /// the TLS handshake. if unspecified, ngrok will choose an industry-safe default.
    /// This value must be null if `terminate_at` is set to `upstream`.
    pub min_version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointRequestHeaders {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// a map of header key to header value that will be injected into the HTTP Request
    /// before being sent to the upstream application server
    pub add: HashMap<String, String>,
    /// a list of header names that will be removed from the HTTP Request before being
    /// sent to the upstream application server
    pub remove: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointResponseHeaders {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// a map of header key to header value that will be injected into the HTTP Response
    /// returned to the HTTP client
    pub add: HashMap<String, String>,
    /// a list of header names that will be removed from the HTTP Response returned to
    /// the HTTP client
    pub remove: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointIPPolicy {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// list of all IP policies that will be used to check if a source IP is allowed
    /// access to the endpoint
    pub ip_policies: Vec<Ref>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointIPPolicyMutate {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// list of all IP policies that will be used to check if a source IP is allowed
    /// access to the endpoint
    pub ip_policy_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointCircuitBreaker {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// Integer number of seconds after which the circuit is tripped to wait before
    /// re-evaluating upstream health
    pub tripped_duration: u32,
    /// Integer number of seconds in the statistical rolling window that metrics are
    /// retained for.
    pub rolling_window: u32,
    /// Integer number of buckets into which metrics are retained. Max 128.
    pub num_buckets: u32,
    /// Integer number of requests in a rolling window that will trip the circuit.
    /// Helpful if traffic volume is low.
    pub volume_threshold: u32,
    /// Error threshold percentage should be between 0 - 1.0, not 0-100.0
    pub error_threshold_percentage: f64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuth {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// an object which defines the identity provider to use for authentication and
    /// configuration for who may access the endpoint
    pub provider: EndpointOAuthProvider,
    /// Do not enforce authentication on HTTP OPTIONS requests. necessary if you are
    /// supporting CORS.
    pub options_passthrough: bool,
    /// the prefix of the session cookie that ngrok sets on the http client to cache
    /// authentication. default is 'ngrok.'
    pub cookie_prefix: String,
    /// Integer number of seconds of inactivity after which if the user has not accessed
    /// the endpoint, their session will time out and they will be forced to
    /// reauthenticate.
    pub inactivity_timeout: u32,
    /// Integer number of seconds of the maximum duration of an authenticated session.
    /// After this period is exceeded, a user must reauthenticate.
    pub maximum_duration: u32,
    /// Integer number of seconds after which ngrok guarantees it will refresh user
    /// state from the identity provider and recheck whether the user is still
    /// authorized to access the endpoint. This is the preferred tunable to use to
    /// enforce a minimum amount of time after which a revoked user will no longer be
    /// able to access the resource.
    pub auth_check_interval: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuthProvider {
    /// configuration for using github as the identity provider
    pub github: Option<EndpointOAuthGitHub>,
    /// configuration for using facebook as the identity provider
    pub facebook: Option<EndpointOAuthFacebook>,
    /// configuration for using microsoft as the identity provider
    pub microsoft: Option<EndpointOAuthMicrosoft>,
    /// configuration for using google as the identity provider
    pub google: Option<EndpointOAuthGoogle>,
    /// configuration for using linkedin as the identity provider
    pub linkedin: Option<EndpointOAuthLinkedIn>,
    /// configuration for using gitlab as the identity provider
    pub gitlab: Option<EndpointOAuthGitLab>,
    /// configuration for using twitch as the identity provider
    pub twitch: Option<EndpointOAuthTwitch>,
    /// configuration for using amazon as the identity provider
    pub amazon: Option<EndpointOAuthAmazon>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuthGitHub {
    /// the OAuth app client ID. retrieve it from the identity provider's dashboard
    /// where you created your own OAuth app. optional. if unspecified, ngrok will use
    /// its own managed oauth application which has additional restrictions. see the
    /// OAuth module docs for more details. if present, client_secret must be present as
    /// well.
    pub client_id: Option<String>,
    /// the OAuth app client secret. retrieve if from the identity provider's dashboard
    /// where you created your own OAuth app. optional, see all of the caveats in the
    /// docs for `client_id`.
    pub client_secret: Option<String>,
    /// a list of provider-specific OAuth scopes with the permissions your OAuth app
    /// would like to ask for. these may not be set if you are using the ngrok-managed
    /// oauth app (i.e. you must pass both `client_id` and `client_secret` to set
    /// scopes)
    pub scopes: Option<Vec<String>>,
    /// a list of email addresses of users authenticated by identity provider who are
    /// allowed access to the endpoint
    pub email_addresses: Option<Vec<String>>,
    /// a list of email domains of users authenticated by identity provider who are
    /// allowed access to the endpoint
    pub email_domains: Option<Vec<String>>,
    /// a list of github teams identifiers. users will be allowed access to the endpoint
    /// if they are a member of any of these teams. identifiers should be in the 'slug'
    /// format qualified with the org name, e.g. `org-name/team-name`
    pub teams: Option<Vec<String>>,
    /// a list of github org identifiers. users who are members of any of the listed
    /// organizations will be allowed access. identifiers should be the organization's
    /// 'slug'
    pub organizations: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuthFacebook {
    /// the OAuth app client ID. retrieve it from the identity provider's dashboard
    /// where you created your own OAuth app. optional. if unspecified, ngrok will use
    /// its own managed oauth application which has additional restrictions. see the
    /// OAuth module docs for more details. if present, client_secret must be present as
    /// well.
    pub client_id: Option<String>,
    /// the OAuth app client secret. retrieve if from the identity provider's dashboard
    /// where you created your own OAuth app. optional, see all of the caveats in the
    /// docs for `client_id`.
    pub client_secret: Option<String>,
    /// a list of provider-specific OAuth scopes with the permissions your OAuth app
    /// would like to ask for. these may not be set if you are using the ngrok-managed
    /// oauth app (i.e. you must pass both `client_id` and `client_secret` to set
    /// scopes)
    pub scopes: Vec<String>,
    /// a list of email addresses of users authenticated by identity provider who are
    /// allowed access to the endpoint
    pub email_addresses: Vec<String>,
    /// a list of email domains of users authenticated by identity provider who are
    /// allowed access to the endpoint
    pub email_domains: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuthMicrosoft {
    /// the OAuth app client ID. retrieve it from the identity provider's dashboard
    /// where you created your own OAuth app. optional. if unspecified, ngrok will use
    /// its own managed oauth application which has additional restrictions. see the
    /// OAuth module docs for more details. if present, client_secret must be present as
    /// well.
    pub client_id: Option<String>,
    /// the OAuth app client secret. retrieve if from the identity provider's dashboard
    /// where you created your own OAuth app. optional, see all of the caveats in the
    /// docs for `client_id`.
    pub client_secret: Option<String>,
    /// a list of provider-specific OAuth scopes with the permissions your OAuth app
    /// would like to ask for. these may not be set if you are using the ngrok-managed
    /// oauth app (i.e. you must pass both `client_id` and `client_secret` to set
    /// scopes)
    pub scopes: Vec<String>,
    /// a list of email addresses of users authenticated by identity provider who are
    /// allowed access to the endpoint
    pub email_addresses: Vec<String>,
    /// a list of email domains of users authenticated by identity provider who are
    /// allowed access to the endpoint
    pub email_domains: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuthGoogle {
    /// the OAuth app client ID. retrieve it from the identity provider's dashboard
    /// where you created your own OAuth app. optional. if unspecified, ngrok will use
    /// its own managed oauth application which has additional restrictions. see the
    /// OAuth module docs for more details. if present, client_secret must be present as
    /// well.
    pub client_id: Option<String>,
    /// the OAuth app client secret. retrieve if from the identity provider's dashboard
    /// where you created your own OAuth app. optional, see all of the caveats in the
    /// docs for `client_id`.
    pub client_secret: Option<String>,
    /// a list of provider-specific OAuth scopes with the permissions your OAuth app
    /// would like to ask for. these may not be set if you are using the ngrok-managed
    /// oauth app (i.e. you must pass both `client_id` and `client_secret` to set
    /// scopes)
    pub scopes: Vec<String>,
    /// a list of email addresses of users authenticated by identity provider who are
    /// allowed access to the endpoint
    pub email_addresses: Vec<String>,
    /// a list of email domains of users authenticated by identity provider who are
    /// allowed access to the endpoint
    pub email_domains: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuthLinkedIn {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scopes: Vec<String>,
    pub email_addresses: Vec<String>,
    pub email_domains: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuthGitLab {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scopes: Vec<String>,
    pub email_addresses: Vec<String>,
    pub email_domains: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuthTwitch {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scopes: Vec<String>,
    pub email_addresses: Vec<String>,
    pub email_domains: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOAuthAmazon {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scopes: Vec<String>,
    pub email_addresses: Vec<String>,
    pub email_domains: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointSAML {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// Do not enforce authentication on HTTP OPTIONS requests. necessary if you are
    /// supporting CORS.
    pub options_passthrough: bool,
    /// the prefix of the session cookie that ngrok sets on the http client to cache
    /// authentication. default is 'ngrok.'
    pub cookie_prefix: String,
    /// Integer number of seconds of inactivity after which if the user has not accessed
    /// the endpoint, their session will time out and they will be forced to
    /// reauthenticate.
    pub inactivity_timeout: u32,
    /// Integer number of seconds of the maximum duration of an authenticated session.
    /// After this period is exceeded, a user must reauthenticate.
    pub maximum_duration: u32,
    /// The full XML IdP EntityDescriptor. Your IdP may provide this to you as a a file
    /// to download or as a URL.
    pub idp_metadata: String,
    /// If true, indicates that whenever we redirect a user to the IdP for
    /// authentication that the IdP must prompt the user for authentication credentials
    /// even if the user already has a valid session with the IdP.
    pub force_authn: bool,
    /// If true, the IdP may initiate a login directly (e.g. the user does not need to
    /// visit the endpoint first and then be redirected). The IdP should set the
    /// `RelayState` parameter to the target URL of the resource they want the user to
    /// be redirected to after the SAML login assertion has been processed.
    pub allow_idp_initiated: Option<bool>,
    /// If present, only users who are a member of one of the listed groups may access
    /// the target endpoint.
    pub authorized_groups: Vec<String>,
    /// The SP Entity's unique ID. This always takes the form of a URL. In ngrok's
    /// implementation, this URL is the same as the metadata URL. This will need to be
    /// specified to the IdP as configuration.
    pub entity_id: String,
    /// The public URL of the SP's Assertion Consumer Service. This is where the IdP
    /// will redirect to during an authentication flow. This will need to be specified
    /// to the IdP as configuration.
    pub assertion_consumer_service_url: String,
    /// The public URL of the SP's Single Logout Service. This is where the IdP will
    /// redirect to during a single logout flow. This will optionally need to be
    /// specified to the IdP as configuration.
    pub single_logout_url: String,
    /// PEM-encoded x.509 certificate of the key pair that is used to sign all SAML
    /// requests that the ngrok SP makes to the IdP. Many IdPs do not support request
    /// signing verification, but we highly recommend specifying this in the IdP's
    /// configuration if it is supported.
    pub request_signing_certificate_pem: String,
    /// A public URL where the SP's metadata is hosted. If an IdP supports dynamic
    /// configuration, this is the URL it can use to retrieve the SP metadata.
    pub metadata_url: String,
    /// Defines the name identifier format the SP expects the IdP to use in its
    /// assertions to identify subjects. If unspecified, a default value of
    /// `urn:oasis:names:tc:SAML:2.0:nameid-format:persistent` will be used. A subset of
    /// the allowed values enumerated by the SAML specification are supported.
    pub nameid_format: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointSAMLMutate {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// Do not enforce authentication on HTTP OPTIONS requests. necessary if you are
    /// supporting CORS.
    pub options_passthrough: bool,
    /// the prefix of the session cookie that ngrok sets on the http client to cache
    /// authentication. default is 'ngrok.'
    pub cookie_prefix: String,
    /// Integer number of seconds of inactivity after which if the user has not accessed
    /// the endpoint, their session will time out and they will be forced to
    /// reauthenticate.
    pub inactivity_timeout: u32,
    /// Integer number of seconds of the maximum duration of an authenticated session.
    /// After this period is exceeded, a user must reauthenticate.
    pub maximum_duration: u32,
    /// The full XML IdP EntityDescriptor. Your IdP may provide this to you as a a file
    /// to download or as a URL.
    pub idp_metadata: String,
    /// If true, indicates that whenever we redirect a user to the IdP for
    /// authentication that the IdP must prompt the user for authentication credentials
    /// even if the user already has a valid session with the IdP.
    pub force_authn: bool,
    /// If true, the IdP may initiate a login directly (e.g. the user does not need to
    /// visit the endpoint first and then be redirected). The IdP should set the
    /// `RelayState` parameter to the target URL of the resource they want the user to
    /// be redirected to after the SAML login assertion has been processed.
    pub allow_idp_initiated: Option<bool>,
    /// If present, only users who are a member of one of the listed groups may access
    /// the target endpoint.
    pub authorized_groups: Vec<String>,
    /// Defines the name identifier format the SP expects the IdP to use in its
    /// assertions to identify subjects. If unspecified, a default value of
    /// `urn:oasis:names:tc:SAML:2.0:nameid-format:persistent` will be used. A subset of
    /// the allowed values enumerated by the SAML specification are supported.
    pub nameid_format: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointOIDC {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// Do not enforce authentication on HTTP OPTIONS requests. necessary if you are
    /// supporting CORS.
    pub options_passthrough: bool,
    /// the prefix of the session cookie that ngrok sets on the http client to cache
    /// authentication. default is 'ngrok.'
    pub cookie_prefix: String,
    /// Integer number of seconds of inactivity after which if the user has not accessed
    /// the endpoint, their session will time out and they will be forced to
    /// reauthenticate.
    pub inactivity_timeout: u32,
    /// Integer number of seconds of the maximum duration of an authenticated session.
    /// After this period is exceeded, a user must reauthenticate.
    pub maximum_duration: u32,
    /// URL of the OIDC "OpenID provider". This is the base URL used for discovery.
    pub issuer: String,
    /// The OIDC app's client ID and OIDC audience.
    pub client_id: String,
    /// The OIDC app's client secret.
    pub client_secret: String,
    /// The set of scopes to request from the OIDC identity provider.
    pub scopes: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointBackend {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// backend to be used to back this endpoint
    pub backend: Ref,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointBackendMutate {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// backend to be used to back this endpoint
    pub backend_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointWebsocketTCPConverter {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointUserAgentFilter {
    pub enabled: Option<bool>,
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointPolicy {
    /// `true` if the module will be applied to traffic, `false` to disable. default
    /// `true` if unspecified
    pub enabled: Option<bool>,
    /// the inbound rules of the traffic policy.
    pub inbound: Vec<EndpointRule>,
    /// the outbound rules on the traffic policy.
    pub outbound: Vec<EndpointRule>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointRule {
    /// cel expressions that filter traffic the policy rule applies to.
    pub expressions: Vec<String>,
    /// the set of actions on a policy rule.
    pub actions: Vec<EndpointAction>,
    /// the name of the rule that is part of the traffic policy.
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointAction {
    /// the type of action on the policy rule.
    pub r#type: String,
    /// the configuration for the action on the policy rule.
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteItem {
    /// unique identifier of this edge
    pub edge_id: String,
    /// unique identifier of this edge route
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPSEdgeRouteCreate {
    /// unique identifier of this edge
    pub edge_id: String,
    /// Type of match to use for this route. Valid values are "exact_path" and
    /// "path_prefix".
    pub match_type: String,
    /// Route selector: "/blog" or "example.com" or "example.com/blog"
    pub r#match: String,
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this edge. Optional, max 4096
    /// bytes.
    pub metadata: String,
    /// backend module configuration or `null`
    pub backend: Option<EndpointBackendMutate>,
    /// ip restriction module configuration or `null`
    pub ip_restriction: Option<EndpointIPPolicyMutate>,
    /// circuit breaker module configuration or `null`
    pub circuit_breaker: Option<EndpointCircuitBreaker>,
    /// compression module configuration or `null`
    pub compression: Option<EndpointCompression>,
    /// request headers module configuration or `null`
    pub request_headers: Option<EndpointRequestHeaders>,
    /// response headers module configuration or `null`
    pub response_headers: Option<EndpointResponseHeaders>,
    /// webhook verification module configuration or `null`
    pub webhook_verification: Option<EndpointWebhookValidation>,
    /// oauth module configuration or `null`
    pub oauth: Option<EndpointOAuth>,
    /// saml module configuration or `null`
    pub saml: Option<EndpointSAMLMutate>,
    /// oidc module configuration or `null`
    pub oidc: Option<EndpointOIDC>,
    /// websocket to tcp adapter configuration or `null`
    pub websocket_tcp_converter: Option<EndpointWebsocketTCPConverter>,
    pub user_agent_filter: Option<EndpointUserAgentFilter>,
    /// the traffic policy associated with this edge or null
    pub policy: Option<EndpointPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPSEdgeRouteUpdate {
    /// unique identifier of this edge
    pub edge_id: String,
    /// unique identifier of this edge route
    pub id: String,
    /// Type of match to use for this route. Valid values are "exact_path" and
    /// "path_prefix".
    pub match_type: String,
    /// Route selector: "/blog" or "example.com" or "example.com/blog"
    pub r#match: String,
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this edge. Optional, max 4096
    /// bytes.
    pub metadata: String,
    /// backend module configuration or `null`
    pub backend: Option<EndpointBackendMutate>,
    /// ip restriction module configuration or `null`
    pub ip_restriction: Option<EndpointIPPolicyMutate>,
    /// circuit breaker module configuration or `null`
    pub circuit_breaker: Option<EndpointCircuitBreaker>,
    /// compression module configuration or `null`
    pub compression: Option<EndpointCompression>,
    /// request headers module configuration or `null`
    pub request_headers: Option<EndpointRequestHeaders>,
    /// response headers module configuration or `null`
    pub response_headers: Option<EndpointResponseHeaders>,
    /// webhook verification module configuration or `null`
    pub webhook_verification: Option<EndpointWebhookValidation>,
    /// oauth module configuration or `null`
    pub oauth: Option<EndpointOAuth>,
    /// saml module configuration or `null`
    pub saml: Option<EndpointSAMLMutate>,
    /// oidc module configuration or `null`
    pub oidc: Option<EndpointOIDC>,
    /// websocket to tcp adapter configuration or `null`
    pub websocket_tcp_converter: Option<EndpointWebsocketTCPConverter>,
    pub user_agent_filter: Option<EndpointUserAgentFilter>,
    /// the traffic policy associated with this edge or null
    pub policy: Option<EndpointPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPSEdgeRoute {
    /// unique identifier of this edge
    pub edge_id: String,
    /// unique identifier of this edge route
    pub id: String,
    /// timestamp when the edge configuration was created, RFC 3339 format
    pub created_at: String,
    /// Type of match to use for this route. Valid values are "exact_path" and
    /// "path_prefix".
    pub match_type: String,
    /// Route selector: "/blog" or "example.com" or "example.com/blog"
    pub r#match: String,
    /// URI of the edge API resource
    pub uri: String,
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this edge. Optional, max 4096
    /// bytes.
    pub metadata: String,
    /// backend module configuration or `null`
    pub backend: Option<EndpointBackend>,
    /// ip restriction module configuration or `null`
    pub ip_restriction: Option<EndpointIPPolicy>,
    /// circuit breaker module configuration or `null`
    pub circuit_breaker: Option<EndpointCircuitBreaker>,
    /// compression module configuration or `null`
    pub compression: Option<EndpointCompression>,
    /// request headers module configuration or `null`
    pub request_headers: Option<EndpointRequestHeaders>,
    /// response headers module configuration or `null`
    pub response_headers: Option<EndpointResponseHeaders>,
    /// webhook verification module configuration or `null`
    pub webhook_verification: Option<EndpointWebhookValidation>,
    /// oauth module configuration or `null`
    pub oauth: Option<EndpointOAuth>,
    /// saml module configuration or `null`
    pub saml: Option<EndpointSAML>,
    /// oidc module configuration or `null`
    pub oidc: Option<EndpointOIDC>,
    /// websocket to tcp adapter configuration or `null`
    pub websocket_tcp_converter: Option<EndpointWebsocketTCPConverter>,
    pub user_agent_filter: Option<EndpointUserAgentFilter>,
    /// the traffic policy associated with this edge or null
    pub policy: Option<EndpointPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPSEdgeList {
    /// the list of all HTTPS Edges on this account
    pub https_edges: Vec<HTTPSEdge>,
    /// URI of the HTTPS Edge list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPSEdgeCreate {
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this edge; optional, max 4096
    /// bytes.
    pub metadata: String,
    /// hostports served by this edge
    pub hostports: Option<Vec<String>>,
    /// edge modules
    pub mutual_tls: Option<EndpointMutualTLSMutate>,
    pub tls_termination: Option<EndpointTLSTerminationAtEdge>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPSEdgeUpdate {
    /// unique identifier of this edge
    pub id: String,
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this edge; optional, max 4096
    /// bytes.
    pub metadata: Option<String>,
    /// hostports served by this edge
    pub hostports: Option<Vec<String>>,
    /// edge modules
    pub mutual_tls: Option<EndpointMutualTLSMutate>,
    pub tls_termination: Option<EndpointTLSTerminationAtEdge>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct HTTPSEdge {
    /// unique identifier of this edge
    pub id: String,
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this edge; optional, max 4096
    /// bytes.
    pub metadata: String,
    /// timestamp when the edge configuration was created, RFC 3339 format
    pub created_at: String,
    /// URI of the edge API resource
    pub uri: String,
    /// hostports served by this edge
    pub hostports: Option<Vec<String>>,
    /// edge modules
    pub mutual_tls: Option<EndpointMutualTLS>,
    pub tls_termination: Option<EndpointTLSTermination>,
    /// routes
    pub routes: Vec<HTTPSEdgeRoute>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeBackendReplace {
    pub id: String,
    pub module: EndpointBackendMutate,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeIPRestrictionReplace {
    pub id: String,
    pub module: EndpointIPPolicyMutate,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeMutualTLSReplace {
    pub id: String,
    pub module: EndpointMutualTLSMutate,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeTLSTerminationReplace {
    pub id: String,
    pub module: EndpointTLSTermination,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeTLSTerminationAtEdgeReplace {
    pub id: String,
    pub module: EndpointTLSTerminationAtEdge,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgePolicyReplace {
    pub id: String,
    pub module: EndpointPolicy,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteBackendReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointBackendMutate,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteIPRestrictionReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointIPPolicyMutate,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteRequestHeadersReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointRequestHeaders,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteResponseHeadersReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointResponseHeaders,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteCompressionReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointCompression,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteCircuitBreakerReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointCircuitBreaker,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteWebhookVerificationReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointWebhookValidation,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteOAuthReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointOAuth,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteSAMLReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointSAMLMutate,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteOIDCReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointOIDC,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteWebsocketTCPConverterReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointWebsocketTCPConverter,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRouteUserAgentFilterReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointUserAgentFilter,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EdgeRoutePolicyReplace {
    pub edge_id: String,
    pub id: String,
    pub module: EndpointPolicy,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TCPEdgeList {
    /// the list of all TCP Edges on this account
    pub tcp_edges: Vec<TCPEdge>,
    /// URI of the TCP Edge list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TCPEdgeCreate {
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this edge. Optional, max 4096
    /// bytes.
    pub metadata: String,
    /// hostports served by this edge
    pub hostports: Option<Vec<String>>,
    /// edge modules
    pub backend: Option<EndpointBackendMutate>,
    pub ip_restriction: Option<EndpointIPPolicyMutate>,
    /// the traffic policy associated with this edge or null
    pub policy: Option<EndpointPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TCPEdgeUpdate {
    /// unique identifier of this edge
    pub id: String,
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this edge. Optional, max 4096
    /// bytes.
    pub metadata: Option<String>,
    /// hostports served by this edge
    pub hostports: Option<Vec<String>>,
    /// edge modules
    pub backend: Option<EndpointBackendMutate>,
    pub ip_restriction: Option<EndpointIPPolicyMutate>,
    /// the traffic policy associated with this edge or null
    pub policy: Option<EndpointPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TCPEdge {
    /// unique identifier of this edge
    pub id: String,
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this edge. Optional, max 4096
    /// bytes.
    pub metadata: String,
    /// timestamp when the edge was created, RFC 3339 format
    pub created_at: String,
    /// URI of the edge API resource
    pub uri: String,
    /// hostports served by this edge
    pub hostports: Option<Vec<String>>,
    /// edge modules
    pub backend: Option<EndpointBackend>,
    pub ip_restriction: Option<EndpointIPPolicy>,
    /// the traffic policy associated with this edge or null
    pub policy: Option<EndpointPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TLSEdgeList {
    /// the list of all TLS Edges on this account
    pub tls_edges: Vec<TLSEdge>,
    /// URI of the TLS Edge list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TLSEdgeCreate {
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this edge. Optional, max 4096
    /// bytes.
    pub metadata: String,
    /// hostports served by this edge
    pub hostports: Option<Vec<String>>,
    /// edge modules
    pub backend: Option<EndpointBackendMutate>,
    pub ip_restriction: Option<EndpointIPPolicyMutate>,
    pub mutual_tls: Option<EndpointMutualTLSMutate>,
    pub tls_termination: Option<EndpointTLSTermination>,
    /// the traffic policy associated with this edge or null
    pub policy: Option<EndpointPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TLSEdgeUpdate {
    /// unique identifier of this edge
    pub id: String,
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this edge. Optional, max 4096
    /// bytes.
    pub metadata: Option<String>,
    /// hostports served by this edge
    pub hostports: Option<Vec<String>>,
    /// edge modules
    pub backend: Option<EndpointBackendMutate>,
    pub ip_restriction: Option<EndpointIPPolicyMutate>,
    pub mutual_tls: Option<EndpointMutualTLSMutate>,
    pub tls_termination: Option<EndpointTLSTermination>,
    /// the traffic policy associated with this edge or null
    pub policy: Option<EndpointPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TLSEdge {
    /// unique identifier of this edge
    pub id: String,
    /// human-readable description of what this edge will be used for; optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this edge. Optional, max 4096
    /// bytes.
    pub metadata: String,
    /// timestamp when the edge configuration was created, RFC 3339 format
    pub created_at: String,
    /// URI of the edge API resource
    pub uri: String,
    /// hostports served by this edge
    pub hostports: Option<Vec<String>>,
    /// edge modules
    pub backend: Option<EndpointBackend>,
    pub ip_restriction: Option<EndpointIPPolicy>,
    pub mutual_tls: Option<EndpointMutualTLS>,
    pub tls_termination: Option<EndpointTLSTermination>,
    /// the traffic policy associated with this edge or null
    pub policy: Option<EndpointPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Endpoint {
    /// unique endpoint resource identifier
    pub id: String,
    /// identifier of the region this endpoint belongs to
    pub region: String,
    /// timestamp when the endpoint was created in RFC 3339 format
    pub created_at: String,
    /// timestamp when the endpoint was updated in RFC 3339 format
    pub updated_at: String,
    /// URL of the hostport served by this endpoint
    pub public_url: String,
    /// protocol served by this endpoint. one of `http`, `https`, `tcp`, or `tls`
    pub proto: String,
    /// hostport served by this endpoint (hostname:port)
    pub hostport: String,
    /// whether the endpoint is `ephemeral` (served directly by an agent-initiated
    /// tunnel) or `edge` (served by an edge)
    pub r#type: String,
    /// user-supplied metadata of the associated tunnel or edge object
    pub metadata: String,
    /// the domain reserved for this endpoint
    pub domain: Option<Ref>,
    /// the address reserved for this endpoint
    pub tcp_addr: Option<Ref>,
    /// the tunnel serving requests to this endpoint, if this is an ephemeral endpoint
    pub tunnel: Option<Ref>,
    /// the edge serving requests to this endpoint, if this is an edge endpoint
    pub edge: Option<Ref>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointList {
    /// the list of all active endpoints on this account
    pub endpoints: Vec<Endpoint>,
    /// URI of the endpoints list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventDestinationCreate {
    /// Arbitrary user-defined machine-readable data of this Event Destination.
    /// Optional, max 4096 bytes.
    pub metadata: String,
    /// Human-readable description of the Event Destination. Optional, max 255 bytes.
    pub description: String,
    /// The output format you would like to serialize events into when sending to their
    /// target. Currently the only accepted value is `JSON`.
    pub format: String,
    /// An object that encapsulates where and how to send your events. An event
    /// destination must contain exactly one of the following objects, leaving the rest
    /// null: `kinesis`, `firehose`, `cloudwatch_logs`, or `s3`.
    pub target: EventTarget,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventDestinationUpdate {
    /// Unique identifier for this Event Destination.
    pub id: String,
    /// Arbitrary user-defined machine-readable data of this Event Destination.
    /// Optional, max 4096 bytes.
    pub metadata: Option<String>,
    /// Human-readable description of the Event Destination. Optional, max 255 bytes.
    pub description: Option<String>,
    /// The output format you would like to serialize events into when sending to their
    /// target. Currently the only accepted value is `JSON`.
    pub format: Option<String>,
    /// An object that encapsulates where and how to send your events. An event
    /// destination must contain exactly one of the following objects, leaving the rest
    /// null: `kinesis`, `firehose`, `cloudwatch_logs`, or `s3`.
    pub target: Option<EventTarget>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventDestination {
    /// Unique identifier for this Event Destination.
    pub id: String,
    /// Arbitrary user-defined machine-readable data of this Event Destination.
    /// Optional, max 4096 bytes.
    pub metadata: String,
    /// Timestamp when the Event Destination was created, RFC 3339 format.
    pub created_at: String,
    /// Human-readable description of the Event Destination. Optional, max 255 bytes.
    pub description: String,
    /// The output format you would like to serialize events into when sending to their
    /// target. Currently the only accepted value is `JSON`.
    pub format: String,
    /// An object that encapsulates where and how to send your events. An event
    /// destination must contain exactly one of the following objects, leaving the rest
    /// null: `kinesis`, `firehose`, `cloudwatch_logs`, or `s3`.
    pub target: EventTarget,
    /// URI of the Event Destination API resource.
    pub uri: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventDestinationList {
    /// The list of all Event Destinations on this account.
    pub event_destinations: Vec<EventDestination>,
    /// URI of the Event Destinations list API resource.
    pub uri: String,
    /// URI of the next page, or null if there is no next page.
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventTarget {
    /// Configuration used to send events to Amazon Kinesis Data Firehose.
    pub firehose: Option<EventTargetFirehose>,
    /// Configuration used to send events to Amazon Kinesis.
    pub kinesis: Option<EventTargetKinesis>,
    /// Configuration used to send events to Amazon CloudWatch Logs.
    pub cloudwatch_logs: Option<EventTargetCloudwatchLogs>,
    /// Configuration used to send events to Datadog.
    pub datadog: Option<EventTargetDatadog>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventTargetFirehose {
    /// Configuration for how to authenticate into your AWS account. Exactly one of
    /// `role` or `creds` should be configured.
    pub auth: AWSAuth,
    /// An Amazon Resource Name specifying the Firehose delivery stream to deposit
    /// events into.
    pub delivery_stream_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventTargetKinesis {
    /// Configuration for how to authenticate into your AWS account. Exactly one of
    /// `role` or `creds` should be configured.
    pub auth: AWSAuth,
    /// An Amazon Resource Name specifying the Kinesis stream to deposit events into.
    pub stream_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventTargetCloudwatchLogs {
    /// Configuration for how to authenticate into your AWS account. Exactly one of
    /// `role` or `creds` should be configured.
    pub auth: AWSAuth,
    /// An Amazon Resource Name specifying the CloudWatch Logs group to deposit events
    /// into.
    pub log_group_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventTargetDatadog {
    /// Datadog API key to use.
    pub api_key: Option<String>,
    /// Tags to send with the event.
    pub ddtags: Option<String>,
    /// Service name to send with the event.
    pub service: Option<String>,
    /// Datadog site to send event to.
    pub ddsite: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AWSAuth {
    /// A role for ngrok to assume on your behalf to deposit events into your AWS
    /// account.
    pub role: Option<AWSRole>,
    /// Credentials to your AWS account if you prefer ngrok to sign in with long-term
    /// access keys.
    pub creds: Option<AWSCredentials>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AWSRole {
    /// An ARN that specifies the role that ngrok should use to deliver to the
    /// configured target.
    pub role_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AWSCredentials {
    /// The ID portion of an AWS access key.
    pub aws_access_key_id: String,
    /// The secret portion of an AWS access key.
    pub aws_secret_access_key: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSubscriptionCreate {
    /// Arbitrary customer supplied information intended to be machine readable.
    /// Optional, max 4096 chars.
    pub metadata: String,
    /// Arbitrary customer supplied information intended to be human readable. Optional,
    /// max 255 chars.
    pub description: String,
    /// Sources containing the types for which this event subscription will trigger
    pub sources: Vec<EventSourceReplace>,
    /// A list of Event Destination IDs which should be used for this Event
    /// Subscription.
    pub destination_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSubscriptionUpdate {
    /// Unique identifier for this Event Subscription.
    pub id: String,
    /// Arbitrary customer supplied information intended to be machine readable.
    /// Optional, max 4096 chars.
    pub metadata: Option<String>,
    /// Arbitrary customer supplied information intended to be human readable. Optional,
    /// max 255 chars.
    pub description: Option<String>,
    /// Sources containing the types for which this event subscription will trigger
    pub sources: Option<Vec<EventSourceReplace>>,
    /// A list of Event Destination IDs which should be used for this Event
    /// Subscription.
    pub destination_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSubscriptionList {
    /// The list of all Event Subscriptions on this account.
    pub event_subscriptions: Vec<EventSubscription>,
    /// URI of the Event Subscriptions list API resource.
    pub uri: String,
    /// URI of next page, or null if there is no next page.
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSubscription {
    /// Unique identifier for this Event Subscription.
    pub id: String,
    /// URI of the Event Subscription API resource.
    pub uri: String,
    /// When the Event Subscription was created (RFC 3339 format).
    pub created_at: String,
    /// Arbitrary customer supplied information intended to be machine readable.
    /// Optional, max 4096 chars.
    pub metadata: String,
    /// Arbitrary customer supplied information intended to be human readable. Optional,
    /// max 255 chars.
    pub description: String,
    /// Sources containing the types for which this event subscription will trigger
    pub sources: Vec<EventSource>,
    /// Destinations to which these events will be sent
    pub destinations: Vec<Ref>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSourceReplace {
    /// Type of event for which an event subscription will trigger
    pub r#type: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSource {
    /// Type of event for which an event subscription will trigger
    pub r#type: String,
    /// URI of the Event Source API resource.
    pub uri: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSourceList {
    /// The list of all Event Sources for an Event Subscription
    pub sources: Vec<EventSource>,
    /// URI of the next page, or null if there is no next page.
    pub uri: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSourceCreate {
    /// The unique identifier for the Event Subscription that this Event Source is
    /// attached to.
    pub subscription_id: String,
    /// Type of event for which an event subscription will trigger
    pub r#type: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSourceUpdate {
    /// The unique identifier for the Event Subscription that this Event Source is
    /// attached to.
    pub subscription_id: String,
    /// Type of event for which an event subscription will trigger
    pub r#type: String,
}

/// This is needed instead of Item because the parameters are different.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSourceItem {
    /// The unique identifier for the Event Subscription that this Event Source is
    /// attached to.
    pub subscription_id: String,
    /// Type of event for which an event subscription will trigger
    pub r#type: String,
}

/// This is needed instead of Paging because the parameters are different. We also don't need the typical pagination params because pagination of this isn't necessary or supported.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EventSourcePaging {
    /// The unique identifier for the Event Subscription that this Event Source is
    /// attached to.
    pub subscription_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPPolicyCreate {
    /// human-readable description of the source IPs of this IP policy. optional, max
    /// 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this IP policy. optional, max
    /// 4096 bytes.
    pub metadata: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPPolicyUpdate {
    pub id: String,
    /// human-readable description of the source IPs of this IP policy. optional, max
    /// 255 bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this IP policy. optional, max
    /// 4096 bytes.
    pub metadata: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPPolicy {
    /// unique identifier for this IP policy
    pub id: String,
    /// URI of the IP Policy API resource
    pub uri: String,
    /// timestamp when the IP policy was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of the source IPs of this IP policy. optional, max
    /// 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this IP policy. optional, max
    /// 4096 bytes.
    pub metadata: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPPolicyList {
    /// the list of all IP policies on this account
    pub ip_policies: Vec<IPPolicy>,
    /// URI of the IP policy list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPPolicyRuleCreate {
    /// human-readable description of the source IPs of this IP rule. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this IP policy rule. optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// an IP or IP range specified in CIDR notation. IPv4 and IPv6 are both supported.
    pub cidr: String,
    /// ID of the IP policy this IP policy rule will be attached to
    pub ip_policy_id: String,
    /// the action to apply to the policy rule, either `allow` or `deny`
    pub action: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPPolicyRuleUpdate {
    pub id: String,
    /// human-readable description of the source IPs of this IP rule. optional, max 255
    /// bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this IP policy rule. optional,
    /// max 4096 bytes.
    pub metadata: Option<String>,
    /// an IP or IP range specified in CIDR notation. IPv4 and IPv6 are both supported.
    pub cidr: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPPolicyRule {
    /// unique identifier for this IP policy rule
    pub id: String,
    /// URI of the IP policy rule API resource
    pub uri: String,
    /// timestamp when the IP policy rule was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of the source IPs of this IP rule. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this IP policy rule. optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// an IP or IP range specified in CIDR notation. IPv4 and IPv6 are both supported.
    pub cidr: String,
    /// object describing the IP policy this IP Policy Rule belongs to
    pub ip_policy: Ref,
    /// the action to apply to the policy rule, either `allow` or `deny`
    pub action: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPPolicyRuleList {
    /// the list of all IP policy rules on this account
    pub ip_policy_rules: Vec<IPPolicyRule>,
    /// URI of the IP policy rule list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPRestrictionCreate {
    /// human-readable description of this IP restriction. optional, max 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this IP restriction. optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// true if the IP restriction will be enforced. if false, only warnings will be
    /// issued
    pub enforced: bool,
    /// the type of IP restriction. this defines what traffic will be restricted with
    /// the attached policies. four values are currently supported: `dashboard`, `api`,
    /// `agent`, and `endpoints`
    pub r#type: String,
    /// the set of IP policy identifiers that are used to enforce the restriction
    pub ip_policy_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPRestrictionUpdate {
    pub id: String,
    /// human-readable description of this IP restriction. optional, max 255 bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this IP restriction. optional,
    /// max 4096 bytes.
    pub metadata: Option<String>,
    /// true if the IP restriction will be enforced. if false, only warnings will be
    /// issued
    pub enforced: Option<bool>,
    /// the set of IP policy identifiers that are used to enforce the restriction
    pub ip_policy_ids: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPRestriction {
    /// unique identifier for this IP restriction
    pub id: String,
    /// URI of the IP restriction API resource
    pub uri: String,
    /// timestamp when the IP restriction was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of this IP restriction. optional, max 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this IP restriction. optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// true if the IP restriction will be enforced. if false, only warnings will be
    /// issued
    pub enforced: bool,
    /// the type of IP restriction. this defines what traffic will be restricted with
    /// the attached policies. four values are currently supported: `dashboard`, `api`,
    /// `agent`, and `endpoints`
    pub r#type: String,
    /// the set of IP policies that are used to enforce the restriction
    pub ip_policies: Vec<Ref>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IPRestrictionList {
    /// the list of all IP restrictions on this account
    pub ip_restrictions: Vec<IPRestriction>,
    /// URI of the IP restrictions list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedAddrCreate {
    /// human-readable description of what this reserved address will be used for
    pub description: String,
    /// arbitrary user-defined machine-readable data of this reserved address. Optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// reserve the address in this geographic ngrok datacenter. Optional, default is
    /// us. (au, eu, ap, us, jp, in, sa)
    pub region: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedAddrUpdate {
    pub id: String,
    /// human-readable description of what this reserved address will be used for
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this reserved address. Optional,
    /// max 4096 bytes.
    pub metadata: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedAddr {
    /// unique reserved address resource identifier
    pub id: String,
    /// URI of the reserved address API resource
    pub uri: String,
    /// timestamp when the reserved address was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of what this reserved address will be used for
    pub description: String,
    /// arbitrary user-defined machine-readable data of this reserved address. Optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// hostname:port of the reserved address that was assigned at creation time
    pub addr: String,
    /// reserve the address in this geographic ngrok datacenter. Optional, default is
    /// us. (au, eu, ap, us, jp, in, sa)
    pub region: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedAddrList {
    /// the list of all reserved addresses on this account
    pub reserved_addrs: Vec<ReservedAddr>,
    /// URI of the reserved address list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedDomainCreate {
    /// hostname of the reserved domain
    pub domain: String,
    /// deprecated: With the launch of the ngrok Global Network domains traffic is now
    /// handled globally. This field applied only to endpoints. Note that agents may
    /// still connect to specific regions. Optional, null by default. (au, eu, ap, us,
    /// jp, in, sa)
    pub region: String,
    /// human-readable description of what this reserved domain will be used for
    pub description: String,
    /// arbitrary user-defined machine-readable data of this reserved domain. Optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// ID of a user-uploaded TLS certificate to use for connections to targeting this
    /// domain. Optional, mutually exclusive with `certificate_management_policy`.
    pub certificate_id: Option<String>,
    /// configuration for automatic management of TLS certificates for this domain, or
    /// null if automatic management is disabled. Optional, mutually exclusive with
    /// `certificate_id`.
    pub certificate_management_policy: Option<ReservedDomainCertPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedDomainUpdate {
    pub id: String,
    /// human-readable description of what this reserved domain will be used for
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this reserved domain. Optional,
    /// max 4096 bytes.
    pub metadata: Option<String>,
    /// ID of a user-uploaded TLS certificate to use for connections to targeting this
    /// domain. Optional, mutually exclusive with `certificate_management_policy`.
    pub certificate_id: Option<String>,
    /// configuration for automatic management of TLS certificates for this domain, or
    /// null if automatic management is disabled. Optional, mutually exclusive with
    /// `certificate_id`.
    pub certificate_management_policy: Option<ReservedDomainCertPolicy>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedDomain {
    /// unique reserved domain resource identifier
    pub id: String,
    /// URI of the reserved domain API resource
    pub uri: String,
    /// timestamp when the reserved domain was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of what this reserved domain will be used for
    pub description: String,
    /// arbitrary user-defined machine-readable data of this reserved domain. Optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// hostname of the reserved domain
    pub domain: String,
    /// deprecated: With the launch of the ngrok Global Network domains traffic is now
    /// handled globally. This field applied only to endpoints. Note that agents may
    /// still connect to specific regions. Optional, null by default. (au, eu, ap, us,
    /// jp, in, sa)
    pub region: String,
    /// DNS CNAME target for a custom hostname, or null if the reserved domain is a
    /// subdomain of an ngrok owned domain (e.g. *.ngrok.app)
    pub cname_target: Option<String>,
    /// object referencing the TLS certificate used for connections to this domain. This
    /// can be either a user-uploaded certificate, the most recently issued automatic
    /// one, or null otherwise.
    pub certificate: Option<Ref>,
    /// configuration for automatic management of TLS certificates for this domain, or
    /// null if automatic management is disabled
    pub certificate_management_policy: Option<ReservedDomainCertPolicy>,
    /// status of the automatic certificate management for this domain, or null if
    /// automatic management is disabled
    pub certificate_management_status: Option<ReservedDomainCertStatus>,
    /// DNS CNAME target for the host _acme-challenge.example.com, where example.com is
    /// your reserved domain name. This is required to issue certificates for wildcard,
    /// non-ngrok reserved domains. Must be null for non-wildcard domains and ngrok
    /// subdomains.
    pub acme_challenge_cname_target: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedDomainList {
    /// the list of all reserved domains on this account
    pub reserved_domains: Vec<ReservedDomain>,
    /// URI of the reserved domain list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedDomainCertPolicy {
    /// certificate authority to request certificates from. The only supported value is
    /// letsencrypt.
    pub authority: String,
    /// type of private key to use when requesting certificates. Defaults to rsa, can be
    /// either rsa or ecdsa.
    pub private_key_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedDomainCertStatus {
    /// timestamp when the next renewal will be requested, RFC 3339 format
    pub renews_at: Option<String>,
    /// status of the certificate provisioning job, or null if the certificiate isn't
    /// being provisioned or renewed
    pub provisioning_job: Option<ReservedDomainCertJob>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReservedDomainCertJob {
    /// if present, an error code indicating why provisioning is failing. It may be
    /// either a temporary condition (INTERNAL_ERROR), or a permanent one the user must
    /// correct (DNS_ERROR).
    pub error_code: Option<String>,
    /// a message describing the current status or error
    pub msg: String,
    /// timestamp when the provisioning job started, RFC 3339 format
    pub started_at: String,
    /// timestamp when the provisioning job will be retried
    pub retries_at: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHCertificateAuthorityCreate {
    /// human-readable description of this SSH Certificate Authority. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this SSH Certificate Authority.
    /// optional, max 4096 bytes.
    pub metadata: String,
    /// the type of private key to generate. one of `rsa`, `ecdsa`, `ed25519`
    pub private_key_type: String,
    /// the type of elliptic curve to use when creating an ECDSA key
    pub elliptic_curve: String,
    /// the key size to use when creating an RSA key. one of `2048` or `4096`
    pub key_size: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHCertificateAuthorityUpdate {
    pub id: String,
    /// human-readable description of this SSH Certificate Authority. optional, max 255
    /// bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this SSH Certificate Authority.
    /// optional, max 4096 bytes.
    pub metadata: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHCertificateAuthority {
    /// unique identifier for this SSH Certificate Authority
    pub id: String,
    /// URI of the SSH Certificate Authority API resource
    pub uri: String,
    /// timestamp when the SSH Certificate Authority API resource was created, RFC 3339
    /// format
    pub created_at: String,
    /// human-readable description of this SSH Certificate Authority. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this SSH Certificate Authority.
    /// optional, max 4096 bytes.
    pub metadata: String,
    /// raw public key for this SSH Certificate Authority
    pub public_key: String,
    /// the type of private key for this SSH Certificate Authority
    pub key_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHCertificateAuthorityList {
    /// the list of all certificate authorities on this account
    pub ssh_certificate_authorities: Vec<SSHCertificateAuthority>,
    /// URI of the certificates authorities list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHCredentialCreate {
    /// human-readable description of who or what will use the ssh credential to
    /// authenticate. Optional, max 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this ssh credential. Optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// optional list of ACL rules. If unspecified, the credential will have no
    /// restrictions. The only allowed ACL rule at this time is the `bind` rule. The
    /// `bind` rule allows the caller to restrict what domains, addresses, and labels
    /// the token is allowed to bind. For example, to allow the token to open a tunnel
    /// on example.ngrok.io your ACL would include the rule `bind:example.ngrok.io`.
    /// Bind rules for domains may specify a leading wildcard to match multiple domains
    /// with a common suffix. For example, you may specify a rule of
    /// `bind:*.example.com` which will allow `x.example.com`, `y.example.com`,
    /// `*.example.com`, etc. Bind rules for labels may specify a wildcard key and/or
    /// value to match multiple labels. For example, you may specify a rule of
    /// `bind:*=example` which will allow `x=example`, `y=example`, etc. A rule of `'*'`
    /// is equivalent to no acl at all and will explicitly permit all actions.
    pub acl: Vec<String>,
    /// the PEM-encoded public key of the SSH keypair that will be used to authenticate
    pub public_key: String,
    /// If supplied at credential creation, ownership will be assigned to the specified
    /// User or Bot. Only admins may specify an owner other than themselves. Defaults to
    /// the authenticated User or Bot.
    pub owner_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHCredentialUpdate {
    pub id: String,
    /// human-readable description of who or what will use the ssh credential to
    /// authenticate. Optional, max 255 bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this ssh credential. Optional,
    /// max 4096 bytes.
    pub metadata: Option<String>,
    /// optional list of ACL rules. If unspecified, the credential will have no
    /// restrictions. The only allowed ACL rule at this time is the `bind` rule. The
    /// `bind` rule allows the caller to restrict what domains, addresses, and labels
    /// the token is allowed to bind. For example, to allow the token to open a tunnel
    /// on example.ngrok.io your ACL would include the rule `bind:example.ngrok.io`.
    /// Bind rules for domains may specify a leading wildcard to match multiple domains
    /// with a common suffix. For example, you may specify a rule of
    /// `bind:*.example.com` which will allow `x.example.com`, `y.example.com`,
    /// `*.example.com`, etc. Bind rules for labels may specify a wildcard key and/or
    /// value to match multiple labels. For example, you may specify a rule of
    /// `bind:*=example` which will allow `x=example`, `y=example`, etc. A rule of `'*'`
    /// is equivalent to no acl at all and will explicitly permit all actions.
    pub acl: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHCredential {
    /// unique ssh credential resource identifier
    pub id: String,
    /// URI of the ssh credential API resource
    pub uri: String,
    /// timestamp when the ssh credential was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of who or what will use the ssh credential to
    /// authenticate. Optional, max 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this ssh credential. Optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// the PEM-encoded public key of the SSH keypair that will be used to authenticate
    pub public_key: String,
    /// optional list of ACL rules. If unspecified, the credential will have no
    /// restrictions. The only allowed ACL rule at this time is the `bind` rule. The
    /// `bind` rule allows the caller to restrict what domains, addresses, and labels
    /// the token is allowed to bind. For example, to allow the token to open a tunnel
    /// on example.ngrok.io your ACL would include the rule `bind:example.ngrok.io`.
    /// Bind rules for domains may specify a leading wildcard to match multiple domains
    /// with a common suffix. For example, you may specify a rule of
    /// `bind:*.example.com` which will allow `x.example.com`, `y.example.com`,
    /// `*.example.com`, etc. Bind rules for labels may specify a wildcard key and/or
    /// value to match multiple labels. For example, you may specify a rule of
    /// `bind:*=example` which will allow `x=example`, `y=example`, etc. A rule of `'*'`
    /// is equivalent to no acl at all and will explicitly permit all actions.
    pub acl: Vec<String>,
    /// If supplied at credential creation, ownership will be assigned to the specified
    /// User or Bot. Only admins may specify an owner other than themselves. Defaults to
    /// the authenticated User or Bot.
    pub owner_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHCredentialList {
    /// the list of all ssh credentials on this account
    pub ssh_credentials: Vec<SSHCredential>,
    /// URI of the ssh credential list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHHostCertificateCreate {
    /// the ssh certificate authority that is used to sign this ssh host certificate
    pub ssh_certificate_authority_id: String,
    /// a public key in OpenSSH Authorized Keys format that this certificate signs
    pub public_key: String,
    /// the list of principals included in the ssh host certificate. This is the list of
    /// hostnames and/or IP addresses that are authorized to serve SSH traffic with this
    /// certificate. Dangerously, if no principals are specified, this certificate is
    /// considered valid for all hosts.
    pub principals: Vec<String>,
    /// The time when the host certificate becomes valid, in RFC 3339 format. Defaults
    /// to the current time if unspecified.
    pub valid_after: String,
    /// The time when this host certificate becomes invalid, in RFC 3339 format. If
    /// unspecified, a default value of one year in the future will be used. The OpenSSH
    /// certificates RFC calls this `valid_before`.
    pub valid_until: String,
    /// human-readable description of this SSH Host Certificate. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this SSH Host Certificate.
    /// optional, max 4096 bytes.
    pub metadata: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHHostCertificateUpdate {
    pub id: String,
    /// human-readable description of this SSH Host Certificate. optional, max 255
    /// bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this SSH Host Certificate.
    /// optional, max 4096 bytes.
    pub metadata: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHHostCertificate {
    /// unique identifier for this SSH Host Certificate
    pub id: String,
    /// URI of the SSH Host Certificate API resource
    pub uri: String,
    /// timestamp when the SSH Host Certificate API resource was created, RFC 3339
    /// format
    pub created_at: String,
    /// human-readable description of this SSH Host Certificate. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this SSH Host Certificate.
    /// optional, max 4096 bytes.
    pub metadata: String,
    /// a public key in OpenSSH Authorized Keys format that this certificate signs
    pub public_key: String,
    /// the key type of the `public_key`, one of `rsa`, `ecdsa` or `ed25519`
    pub key_type: String,
    /// the ssh certificate authority that is used to sign this ssh host certificate
    pub ssh_certificate_authority_id: String,
    /// the list of principals included in the ssh host certificate. This is the list of
    /// hostnames and/or IP addresses that are authorized to serve SSH traffic with this
    /// certificate. Dangerously, if no principals are specified, this certificate is
    /// considered valid for all hosts.
    pub principals: Vec<String>,
    /// the time when the ssh host certificate becomes valid, in RFC 3339 format.
    pub valid_after: String,
    /// the time after which the ssh host certificate becomes invalid, in RFC 3339
    /// format. the OpenSSH certificates RFC calls this `valid_before`.
    pub valid_until: String,
    /// the signed SSH certificate in OpenSSH Authorized Keys format. this value should
    /// be placed in a `-cert.pub` certificate file on disk that should be referenced in
    /// your `sshd_config` configuration file with a `HostCertificate` directive
    pub certificate: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHHostCertificateList {
    /// the list of all ssh host certificates on this account
    pub ssh_host_certificates: Vec<SSHHostCertificate>,
    /// URI of the ssh host certificates list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHUserCertificateCreate {
    /// the ssh certificate authority that is used to sign this ssh user certificate
    pub ssh_certificate_authority_id: String,
    /// a public key in OpenSSH Authorized Keys format that this certificate signs
    pub public_key: String,
    /// the list of principals included in the ssh user certificate. This is the list of
    /// usernames that the certificate holder may sign in as on a machine authorizing
    /// the signing certificate authority. Dangerously, if no principals are specified,
    /// this certificate may be used to log in as any user.
    pub principals: Vec<String>,
    /// A map of critical options included in the certificate. Only two critical options
    /// are currently defined by OpenSSH: `force-command` and `source-address`. See [the
    /// OpenSSH certificate protocol
    /// spec](https://github.com/openssh/openssh-portable/blob/master/PROTOCOL.certkeys)
    /// for additional details.
    pub critical_options: HashMap<String, String>,
    /// A map of extensions included in the certificate. Extensions are additional
    /// metadata that can be interpreted by the SSH server for any purpose. These can be
    /// used to permit or deny the ability to open a terminal, do port forwarding, x11
    /// forwarding, and more. If unspecified, the certificate will include limited
    /// permissions with the following extension map: `{"permit-pty": "",
    /// "permit-user-rc": ""}` OpenSSH understands a number of predefined extensions.
    /// See [the OpenSSH certificate protocol
    /// spec](https://github.com/openssh/openssh-portable/blob/master/PROTOCOL.certkeys)
    /// for additional details.
    pub extensions: HashMap<String, String>,
    /// The time when the user certificate becomes valid, in RFC 3339 format. Defaults
    /// to the current time if unspecified.
    pub valid_after: String,
    /// The time when this host certificate becomes invalid, in RFC 3339 format. If
    /// unspecified, a default value of 24 hours will be used. The OpenSSH certificates
    /// RFC calls this `valid_before`.
    pub valid_until: String,
    /// human-readable description of this SSH User Certificate. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this SSH User Certificate.
    /// optional, max 4096 bytes.
    pub metadata: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHUserCertificateUpdate {
    pub id: String,
    /// human-readable description of this SSH User Certificate. optional, max 255
    /// bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this SSH User Certificate.
    /// optional, max 4096 bytes.
    pub metadata: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHUserCertificate {
    /// unique identifier for this SSH User Certificate
    pub id: String,
    /// URI of the SSH User Certificate API resource
    pub uri: String,
    /// timestamp when the SSH User Certificate API resource was created, RFC 3339
    /// format
    pub created_at: String,
    /// human-readable description of this SSH User Certificate. optional, max 255
    /// bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this SSH User Certificate.
    /// optional, max 4096 bytes.
    pub metadata: String,
    /// a public key in OpenSSH Authorized Keys format that this certificate signs
    pub public_key: String,
    /// the key type of the `public_key`, one of `rsa`, `ecdsa` or `ed25519`
    pub key_type: String,
    /// the ssh certificate authority that is used to sign this ssh user certificate
    pub ssh_certificate_authority_id: String,
    /// the list of principals included in the ssh user certificate. This is the list of
    /// usernames that the certificate holder may sign in as on a machine authorizing
    /// the signing certificate authority. Dangerously, if no principals are specified,
    /// this certificate may be used to log in as any user.
    pub principals: Vec<String>,
    /// A map of critical options included in the certificate. Only two critical options
    /// are currently defined by OpenSSH: `force-command` and `source-address`. See [the
    /// OpenSSH certificate protocol
    /// spec](https://github.com/openssh/openssh-portable/blob/master/PROTOCOL.certkeys)
    /// for additional details.
    pub critical_options: HashMap<String, String>,
    /// A map of extensions included in the certificate. Extensions are additional
    /// metadata that can be interpreted by the SSH server for any purpose. These can be
    /// used to permit or deny the ability to open a terminal, do port forwarding, x11
    /// forwarding, and more. If unspecified, the certificate will include limited
    /// permissions with the following extension map: `{"permit-pty": "",
    /// "permit-user-rc": ""}` OpenSSH understands a number of predefined extensions.
    /// See [the OpenSSH certificate protocol
    /// spec](https://github.com/openssh/openssh-portable/blob/master/PROTOCOL.certkeys)
    /// for additional details.
    pub extensions: HashMap<String, String>,
    /// the time when the ssh host certificate becomes valid, in RFC 3339 format.
    pub valid_after: String,
    /// the time after which the ssh host certificate becomes invalid, in RFC 3339
    /// format. the OpenSSH certificates RFC calls this `valid_before`.
    pub valid_until: String,
    /// the signed SSH certificate in OpenSSH Authorized Keys Format. this value should
    /// be placed in a `-cert.pub` certificate file on disk that should be referenced in
    /// your `sshd_config` configuration file with a `HostCertificate` directive
    pub certificate: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SSHUserCertificateList {
    /// the list of all ssh user certificates on this account
    pub ssh_user_certificates: Vec<SSHUserCertificate>,
    /// URI of the ssh user certificates list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TLSCertificateCreate {
    /// human-readable description of this TLS certificate. optional, max 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this TLS certificate. optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// chain of PEM-encoded certificates, leaf first. See [Certificate
    /// Bundles](https://ngrok.com/docs/cloud-edge/endpoints#certificate-chains).
    pub certificate_pem: String,
    /// private key for the TLS certificate, PEM-encoded. See [Private
    /// Keys](https://ngrok.com/docs/cloud-edge/endpoints#private-keys).
    pub private_key_pem: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TLSCertificateUpdate {
    pub id: String,
    /// human-readable description of this TLS certificate. optional, max 255 bytes.
    pub description: Option<String>,
    /// arbitrary user-defined machine-readable data of this TLS certificate. optional,
    /// max 4096 bytes.
    pub metadata: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TLSCertificate {
    /// unique identifier for this TLS certificate
    pub id: String,
    /// URI of the TLS certificate API resource
    pub uri: String,
    /// timestamp when the TLS certificate was created, RFC 3339 format
    pub created_at: String,
    /// human-readable description of this TLS certificate. optional, max 255 bytes.
    pub description: String,
    /// arbitrary user-defined machine-readable data of this TLS certificate. optional,
    /// max 4096 bytes.
    pub metadata: String,
    /// chain of PEM-encoded certificates, leaf first. See [Certificate
    /// Bundles](https://ngrok.com/docs/cloud-edge/endpoints#certificate-chains).
    pub certificate_pem: String,
    /// subject common name from the leaf of this TLS certificate
    pub subject_common_name: String,
    /// subject alternative names (SANs) from the leaf of this TLS certificate
    pub subject_alternative_names: TLSCertificateSANs,
    /// timestamp (in RFC 3339 format) when this TLS certificate was issued
    /// automatically, or null if this certificate was user-uploaded
    pub issued_at: Option<String>,
    /// timestamp when this TLS certificate becomes valid, RFC 3339 format
    pub not_before: String,
    /// timestamp when this TLS certificate becomes invalid, RFC 3339 format
    pub not_after: String,
    /// set of actions the private key of this TLS certificate can be used for
    pub key_usages: Vec<String>,
    /// extended set of actions the private key of this TLS certificate can be used for
    pub extended_key_usages: Vec<String>,
    /// type of the private key of this TLS certificate. One of rsa, ecdsa, or ed25519.
    pub private_key_type: String,
    /// issuer common name from the leaf of this TLS certificate
    pub issuer_common_name: String,
    /// serial number of the leaf of this TLS certificate
    pub serial_number: String,
    /// subject organization from the leaf of this TLS certificate
    pub subject_organization: String,
    /// subject organizational unit from the leaf of this TLS certificate
    pub subject_organizational_unit: String,
    /// subject locality from the leaf of this TLS certificate
    pub subject_locality: String,
    /// subject province from the leaf of this TLS certificate
    pub subject_province: String,
    /// subject country from the leaf of this TLS certificate
    pub subject_country: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TLSCertificateList {
    /// the list of all TLS certificates on this account
    pub tls_certificates: Vec<TLSCertificate>,
    /// URI of the TLS certificates list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TLSCertificateSANs {
    /// set of additional domains (including wildcards) this TLS certificate is valid
    /// for
    pub dns_names: Vec<String>,
    /// set of IP addresses this TLS certificate is also valid for
    pub ips: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Tunnel {
    /// unique tunnel resource identifier
    pub id: String,
    /// URL of the ephemeral tunnel's public endpoint
    pub public_url: String,
    /// timestamp when the tunnel was initiated in RFC 3339 format
    pub started_at: String,
    /// user-supplied metadata for the tunnel defined in the ngrok configuration file.
    /// See the tunnel [metadata configuration
    /// option](https://ngrok.com/docs/secure-tunnels/ngrok-agent/reference/config#common-tunnel-configuration-properties)
    /// In API version 0, this value was instead pulled from the top-level [metadata
    /// configuration
    /// option](https://ngrok.com/docs/secure-tunnels/ngrok-agent/reference/config#metadata).
    pub metadata: String,
    /// tunnel protocol for ephemeral tunnels. one of `http`, `https`, `tcp` or `tls`
    pub proto: String,
    /// identifier of tune region where the tunnel is running
    pub region: String,
    /// reference object pointing to the tunnel session on which this tunnel was started
    pub tunnel_session: Ref,
    /// the ephemeral endpoint this tunnel is associated with, if this is an
    /// agent-initiated tunnel
    pub endpoint: Option<Ref>,
    /// the labels the tunnel group backends will match against, if this is a backend
    /// tunnel
    pub labels: Option<HashMap<String, String>>,
    /// tunnel group backends served by this backend tunnel
    pub backends: Option<Vec<Ref>>,
    /// upstream address the ngrok agent forwards traffic over this tunnel to. this may
    /// be expressed as a URL or a network address.
    pub forwards_to: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TunnelList {
    /// the list of all online tunnels on this account
    pub tunnels: Vec<Tunnel>,
    /// URI of the tunnels list API resource
    pub uri: String,
    /// URI of the next page, or null if there is no next page
    pub next_page_uri: Option<String>,
}
