use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VmessItem {
    #[serde(rename = "indexId")]
    index_id: String,
    #[serde(rename = "configType")]
    config_type: u8,
    sort: u8,
    address: String,
    port: u16,
    id: String,
    #[serde(rename = "alterId")]
    alter_id: u32,
    security: String,
    network: String,
    remarks: String,
    #[serde(rename = "headerType")]
    header_type: String,
    #[serde(rename = "requestHost")]
    request_host: String,
    path: String,
    #[serde(rename = "streamSecurity")]
    stream_security: String,
    #[serde(rename = "allowInsecure")]
    allow_insecure: String,
    #[serde(rename = "testResult")]
    test_result: String,
    subid: String,
    flow: String,
    #[serde(rename = "groupId")]
    group_id: String,
}