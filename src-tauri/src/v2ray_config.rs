use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct V2rayConfig {
    log: Log,
    inbounds: Vec<Inbounds>,
    outbounds: Vec<Outbounds>,
    stats: Stats,
    api: Api,
    policy: Policy,
    dns: Option<Dns>,
    routing: Routing,
}

#[derive(Debug, Serialize, Deserialize)]
struct Log {
    access: String,
    error: String,
    #[serde(rename = "loglevel")]
    log_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sniffing {
    enabled: bool,
    #[serde(rename = "destOverride")]
    dest_override: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsersItem {
    id: String,
    #[serde(rename = "alterId")]
    alter_id: i32,
    email: String,
    security: String,
    encryption: String,
    flow: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AccountsItem {
    user: String,
    pass: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InboundSettings {
    auth: Option<String>,
    udp: bool,
    ip: Option<String>,
    address: Option<String>,
    clients: Option<Vec<UsersItem>>,
    decryption: Option<String>,
    #[serde(rename = "allowTransparent")]
    allow_transparent: bool,
    accounts: Option<Vec<AccountsItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TlsSettings {
    #[serde(rename = "allowInsecure")]
    allow_insecure: bool,
    #[serde(rename = "serverName")]
    server_name: Option<String>,
    alpn: Vec<String>,
    #[serde(rename = "fingerprint")]
    finger_print: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Header {
    #[serde(rename = "type")]
    m_type: String,
    // request: T,
    // response: U,
}

#[derive(Debug, Serialize, Deserialize)]
struct TcpSettings {
    header: Header,
}

#[derive(Debug, Serialize, Deserialize)]
struct KcpSettings {
    mtu: i32,
    tti: i32,
    #[serde(rename = "uplinkCapacity")]
    uplink_capacity: i32,
    #[serde(rename = "downlinkCapacity")]
    downlink_capacity: i32,
    congestion: i32,
    #[serde(rename = "readBufferSize")]
    read_buffer_size: i32,
    #[serde(rename = "writeBufferSize")]
    write_buffer_size: i32,
    header: Header,
    seed: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Headers {
    #[serde(rename = "Host")]
    host: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WsSettings {
    path: String,
    headers: Option<Headers>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HttpSettings {
    path: String,
    host: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QuicSettings {
    security: String,
    key: String,
    header: Header,
}

#[derive(Debug, Serialize, Deserialize)]
struct GrpcSettings {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "multiMode")]
    multi_mode: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct StreamSettings {
    network: String,
    security: String,
    #[serde(rename = "tlsSettings")]
    tls_settings: Option<TlsSettings>,
    #[serde(rename = "tcpSettings")]
    tcp_settings: Option<TcpSettings>,
    #[serde(rename = "kcpSettings")]
    kcp_settings: Option<KcpSettings>,
    #[serde(rename = "wsSettings")]
    ws_settings: Option<WsSettings>,
    #[serde(rename = "httpSettings")]
    http_settings: Option<HttpSettings>,
    #[serde(rename = "quicSettings")]
    quic_settings: Option<QuicSettings>,
    #[serde(rename = "xtlsSettings")]
    xtls_settings: Option<TlsSettings>,
    #[serde(rename = "grpcSettings")]
    grpc_settings: Option<GrpcSettings>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Inbounds {
    tag: String,
    port: i32,
    listen: String,
    protocol: String,
    sniffing: Option<Sniffing>,
    settings: InboundSettings,
    #[serde(rename = "streamSettings")]
    stream_settings: Option<StreamSettings>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Outbounds {
    tag: String,
    protocol: String,
    settings: OutboundSettings,
    #[serde(rename = "streamSettings")]
    stream_settings: Option<StreamSettings>,
    mux: Option<Mux>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutboundSettings {
    vnext: Option<Vec<VnextItem>>,
    servers: Option<Vec<ServersItem>>,
    response: Option<Response>,
    #[serde(rename = "domainStrategy")]
    domain_strategy: Option<String>,
    #[serde(rename = "userLevel")]
    user_level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VnextItem {
    address: String,
    port: i32,
    users: Vec<UsersItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServersItem {
    email: String,
    address: String,
    method: String,
    ota: bool,
    password: String,
    port: i32,
    level: i32,
    flow: String,
    users: Vec<SocksUsersItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SocksUsersItem {
    user: String,
    pass: String,
    level: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Mux {
    enabled: bool,
    concurrency: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "type")]
    m_type: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Dns {
    servers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Routing {
    #[serde(rename = "domainStrategy")]
    domain_strategy: String,
    #[serde(rename = "domainMatcher")]
    domain_matcher: String,
    rules: Vec<RulesItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RulesItem {
    #[serde(rename = "type")]
    m_type: String,
    port: Option<String>,
    #[serde(rename = "inboundTag")]
    inbound_tag: Option<Vec<String>>,
    #[serde(rename = "outboundTag")]
    outbound_tag: String,
    ip: Option<Vec<String>>,
    domain: Option<Vec<String>>,
    protocol: Option<Vec<String>>,
    enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stats {}

#[derive(Debug, Serialize, Deserialize)]
struct Api {
    tag: String,
    services: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Policy {
    system: SystemPolicy,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemPolicy {
    #[serde(rename = "statsOutboundUplink")]
    stats_outbound_uplink: bool,
    #[serde(rename = "statsOutboundDownlink")]
    stats_outbound_downlink: bool,
}
