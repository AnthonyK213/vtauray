export interface LogPayload {
  m_type: number,
  message: string,
}

export interface StatsPayload {
    outbound_proxy_traffic_downlink_speed: string,
}

export interface InboundItem {
  localPort: number,
  protocol: string,
}

export interface VmessItem {
  indexId: string,
  configType: number,
  configVersion: number,
  sort: number,
  address: string,
  port: number,
  id: string,
  alterId: number,
  security: string,
  network: string,
  remarks: string,
  headerType: string,
  requestHost: string,
  path: string,
  streamSecurity: string,
  allowInsecure: string,
  testResult: string,
  subid: string,
  flow: string,
  sni: string,
  fingerprint: string,
  publicKey: string,
  shortId: string,
  spiderX: string,
  alpn: string[],
  groupId: string,
  coreType: number,
  preSocksPort: number,
}

export interface VAppConfig {
  indexId: string,
  inbound: InboundItem[],
  vmess: VmessItem[],
}
