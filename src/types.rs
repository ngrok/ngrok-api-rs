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
pub struct ItemPaging {
    /// a resource identifier
    pub id: String,
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
    pub scheme: String,
    /// hostport served by this endpoint (hostname:port) -> soon to be deprecated
    pub hostport: String,
    pub host: String,
    pub port: i64,
    /// whether the endpoint is `ephemeral` (served directly by an agent-initiated
    /// tunnel) or `edge` (served by an edge) or `cloud (represents a cloud endpoint)`
    pub r#type: String,
    /// user-supplied metadata of the associated tunnel or edge object
    pub metadata: String,
    /// user-supplied description of the associated tunnel
    pub description: String,
    /// the domain reserved for this endpoint
    pub domain: Option<Ref>,
    /// the address reserved for this endpoint
    pub tcp_addr: Option<Ref>,
    /// the tunnel serving requests to this endpoint, if this is an ephemeral endpoint
    pub tunnel: Option<Ref>,
    /// the edge serving requests to this endpoint, if this is an edge endpoint
    pub edge: Option<Ref>,
    /// the local address the tunnel forwards to
    pub upstream_url: String,
    /// the protocol the agent uses to forward with
    pub upstream_protocol: String,
    /// the url of the endpoint
    pub url: String,
    /// The ID of the owner (bot or user) that owns this endpoint
    pub principal: Option<Ref>,
    /// The traffic policy attached to this endpoint
    pub traffic_policy: String,
    /// the bindings associated with this endpoint
    pub bindings: Option<Vec<String>>,
    /// The tunnel session of the agent for this endpoint
    pub tunnel_session: Option<Ref>,
    /// URI of the Cloud Endpoint API resource
    pub uri: String,
    /// user supplied name for the endpoint
    pub name: String,
    /// whether the endpoint allows pooling
    pub pooling_enabled: bool,
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
pub struct EndpointCreate {
    /// the url of the endpoint
    pub url: String,
    /// Type of endpoint. Only 'cloud' is currently supported (represents a cloud
    /// endpoint). Defaults to 'cloud' if not specified.
    pub r#type: String,
    /// The traffic policy attached to this endpoint
    pub traffic_policy: String,
    /// user-supplied description of the associated tunnel
    pub description: Option<String>,
    /// user-supplied metadata of the associated tunnel or edge object
    pub metadata: Option<String>,
    /// the bindings associated with this endpoint
    pub bindings: Option<Vec<String>>,
    pub pooling_enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointListArgs {
    pub before_id: Option<String>,
    pub limit: Option<String>,
    pub id: Vec<String>,
    pub url: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EndpointUpdate {
    /// unique endpoint resource identifier
    pub id: String,
    /// the url of the endpoint
    pub url: Option<String>,
    /// The traffic policy attached to this endpoint
    pub traffic_policy: Option<String>,
    /// user-supplied description of the associated tunnel
    pub description: Option<String>,
    /// user-supplied metadata of the associated tunnel or edge object
    pub metadata: Option<String>,
    /// the bindings associated with this endpoint
    pub bindings: Option<Vec<String>>,
    pub pooling_enabled: bool,
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
    pub azure_logs_ingestion: Option<EventTargetAzureLogsIngestion>,
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
pub struct EventTargetAzureLogsIngestion {
    /// Tenant ID for the Azure account
    pub tenant_id: String,
    /// Client ID for the application client
    pub client_id: String,
    /// Client Secret for the application client
    pub client_secret: String,
    /// Data collection endpoint logs ingestion URI
    pub logs_ingestion_uri: String,
    /// Data collection rule immutable ID
    pub data_collection_rule_id: String,
    /// Data collection stream name to use as destination, located inside the DCR
    pub data_collection_stream_name: String,
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
    /// type of private key to use when requesting certificates. Defaults to ecdsa, can
    /// be either rsa or ecdsa.
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
pub struct SecretCreate {
    /// Name of secret
    pub name: String,
    /// Value of secret
    pub value: String,
    /// Arbitrary user-defined metadata for this Secret
    pub metadata: String,
    /// description of Secret
    pub description: String,
    /// unique identifier of the referenced vault
    pub vault_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SecretUpdate {
    /// identifier for Secret
    pub id: String,
    /// Name of secret
    pub name: Option<String>,
    /// Value of secret
    pub value: Option<String>,
    /// Arbitrary user-defined metadata for this Secret
    pub metadata: Option<String>,
    /// description of Secret
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Secret {
    /// identifier for Secret
    pub id: String,
    /// URI of this Secret API resource
    pub uri: String,
    /// Timestamp when the Secret was created (RFC 3339 format)
    pub created_at: String,
    /// Timestamp when the Secret was last updated (RFC 3339 format)
    pub updated_at: String,
    /// Name of secret
    pub name: String,
    /// description of Secret
    pub description: String,
    /// Arbitrary user-defined metadata for this Secret
    pub metadata: String,
    /// Reference to who created this Secret
    pub created_by: Ref,
    /// Reference to who created this Secret
    pub last_updated_by: Ref,
    /// Reference to the vault the secret is stored in
    pub vault: Ref,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SecretList {
    /// The list of Secrets for this account
    pub secrets: Vec<Secret>,
    pub uri: String,
    /// URI of the next page of results, or null if there is no next page
    pub next_page_uri: Option<String>,
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VaultCreate {
    /// Name of vault
    pub name: String,
    /// Arbitrary user-defined metadata for this Vault
    pub metadata: String,
    /// description of Vault
    pub description: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VaultUpdate {
    /// identifier for Vault
    pub id: String,
    /// Name of vault
    pub name: Option<String>,
    /// Arbitrary user-defined metadata for this Vault
    pub metadata: Option<String>,
    /// description of Vault
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Vault {
    /// identifier for Vault
    pub id: String,
    /// URI of this Vault API resource
    pub uri: String,
    /// Timestamp when the Vault was created (RFC 3339 format)
    pub created_at: String,
    /// Timestamp when the Vault was last updated (RFC 3339 format)
    pub updated_at: String,
    /// Name of vault
    pub name: String,
    /// description of Vault
    pub description: String,
    /// Arbitrary user-defined metadata for this Vault
    pub metadata: String,
    /// Reference to who created this Vault
    pub created_by: String,
    /// Reference to who created this Vault
    pub last_updated_by: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VaultList {
    /// The list of Vaults for this account
    pub vaults: Vec<Vault>,
    pub uri: String,
    /// URI of the next page of results, or null if there is no next page
    pub next_page_uri: Option<String>,
}
