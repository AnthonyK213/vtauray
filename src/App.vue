<script setup lang="ts">
import { Event, listen } from '@tauri-apps/api/event';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { exit } from '@tauri-apps/api/process';
import { configDir } from '@tauri-apps/api/path';
import { confirm } from '@tauri-apps/api/dialog';
import { ref, onMounted, nextTick } from 'vue';
import { VAppConfig, VmessItem } from './interface'
import { vConnect, vDisconnect, xUpdate } from './lib'
import Helm from './components/Helm.vue'
import StatusLine from './components/StatusLine.vue';
import Log from './components/Log.vue';

let vAppConfigPath: string
let vConfigPath: string;
let vAppConfig: VAppConfig;
const vServers = ref<VmessItem[]>([]);
const vServerSelect = ref<HTMLSelectElement | null>(null);

const vServerRemarks = ref('');
const vServerAddress = ref('');
const vServerPort = ref(443);
const vServerId = ref('');
const vServerFlow = ref('');
const vServerNetwork = ref('');
const vServerPath = ref('');
const vServerStreamSecurity = ref('');
const vServerSni = ref('');
const vServerFingerPrint = ref('');
const vServerPublicKey = ref('');
const vServerShortId = ref('');
const vServerSpiderX = ref('');
const vServerAllowInsecure = ref('');
const vServerAlpn = ref('');

const vAppConfigPort = ref(7890);

async function v2rayConfig() {
  if (!vServerSelect.value || vServerSelect.value.selectedIndex < 0) {
    return;
  }
  let server = vServers.value[vServerSelect.value.selectedIndex];
  let config = {
    "policy": {
      "system": {
        "statsInboundDownlink": true,
        "statsInboundUplink": true,
        "statsOutboundDownlink": true,
        "statsOutboundUplink": true
      }
    },
    "log": {
      "access": "",
      "error": "",
      "loglevel": "warning"
    },
    "inbounds": [
      {
        "tag": "socks",
        "port": vAppConfig.inbound[0].localPort,
        "listen": "127.0.0.1",
        "protocol": "socks",
        "sniffing": {
          "enabled": true,
          "destOverride": [
            "http",
            "tls"
          ]
        },
        "settings": {
          "auth": "noauth",
          "udp": true,
          "allowTransparent": false
        }
      },
      {
        "tag": "http",
        "port": vAppConfig.inbound[0].localPort + 1,
        "listen": "127.0.0.1",
        "protocol": "http",
        "sniffing": {
          "enabled": true,
          "destOverride": [
            "http",
            "tls"
          ]
        },
        "settings": {
          "auth": "noauth",
          "udp": true,
          "allowTransparent": false
        }
      },
      {
        "tag": "socks2",
        "port": vAppConfig.inbound[0].localPort + 2,
        "listen": "0.0.0.0",
        "protocol": "socks",
        "sniffing": {
          "enabled": true,
          "destOverride": [
            "http",
            "tls"
          ]
        },
        "settings": {
          "auth": "noauth",
          "udp": true,
          "allowTransparent": false
        }
      },
      {
        "tag": "http2",
        "port": vAppConfig.inbound[0].localPort + 3,
        "listen": "0.0.0.0",
        "protocol": "http",
        "sniffing": {
          "enabled": true,
          "destOverride": [
            "http",
            "tls"
          ]
        },
        "settings": {
          "auth": "noauth",
          "udp": true,
          "allowTransparent": false
        }
      },
      {
        "tag": "api",
        "port": 49723,
        "listen": "127.0.0.1",
        "protocol": "dokodemo-door",
        "settings": {
          "udp": false,
          "address": "127.0.0.1",
          "allowTransparent": false
        }
      }
    ],
    "outbounds": [
      {
        "tag": "proxy",
        "protocol": "vless",
        "settings": {
          "vnext": [
            {
              "address": server.address,
              "port": server.port,
              "users": [
                {
                  "id": server.id,
                  "alterId": server.alterId,
                  "email": "t@t.tt",
                  "security": "auto",
                  "encryption": "none",
                  "flow": ""
                }
              ]
            }
          ]
        },
        "streamSettings": {
          "network": server.network,
          "security": server.streamSecurity,
          "tlsSettings": {
            "allowInsecure": server.allowInsecure === "true",
            "alpn": server.alpn
          },
          "wsSettings": {
            "path": server.path
          }
        },
        "mux": {
          "enabled": false,
          "concurrency": -1
        }
      },
      {
        "tag": "direct",
        "protocol": "freedom",
        "settings": {}
      },
      {
        "tag": "block",
        "protocol": "blackhole",
        "settings": {
          "response": {
            "type": "http"
          }
        }
      }
    ],
    "stats": {},
    "api": {
      "tag": "api",
      "services": [
        "StatsService"
      ]
    },
    "routing": {
      "domainStrategy": "IPIfNonMatch",
      "domainMatcher": "linear",
      "rules": [
        {
          "type": "field",
          "inboundTag": [
            "api"
          ],
          "outboundTag": "api",
          "enabled": true
        },
        {
          "type": "field",
          "inboundTag": [],
          "outboundTag": "direct",
          "domain": [
            "domain:example-example.com",
            "domain:example-example2.com"
          ],
          "enabled": true
        },
        {
          "type": "field",
          "inboundTag": [],
          "outboundTag": "block",
          "domain": [
            "geosite:category-ads-all"
          ],
          "enabled": true
        },
        {
          "type": "field",
          "outboundTag": "direct",
          "domain": [
            "geosite:cn"
          ],
          "enabled": true
        },
        {
          "type": "field",
          "outboundTag": "direct",
          "ip": [
            "geoip:private",
            "geoip:cn"
          ],
          "enabled": true
        },
        {
          "type": "field",
          "port": "0-65535",
          "outboundTag": "proxy",
          "enabled": true
        }
      ]
    }
  };

  await writeTextFile(vConfigPath, JSON.stringify(config));
}

async function v2rayConnect() {
  await v2rayConfig();
  await vConnect(vConfigPath);
}

async function v2rayDisconnect() {
  await vDisconnect();
}

async function serverAdd() {
  vServers.value.push({
    "indexId": Date.parse(new Date().toString()).toString(),
    "configType": 5,
    "configVersion": 2,
    "sort": 0,
    "address": "",
    "port": 443,
    "id": "",
    "alterId": 0,
    "security": "none",
    "network": "",
    "remarks": "Unnamed",
    "headerType": "",
    "requestHost": "",
    "path": "",
    "streamSecurity": "",
    "allowInsecure": "",
    "testResult": "",
    "subid": "",
    "flow": "",
    "sni": "",
    "fingerprint": "",
    "publicKey": "",
    "shortId": "",
    "spiderX": "",
    "alpn": [
      ""
    ],
    "groupId": "",
    "coreType": 2,
    "preSocksPort": 0
  });
  if (vServerSelect.value) {
    await nextTick();
    vServerSelect.value.selectedIndex = vServers.value.length - 1;
    onSelectedServerChanged();
  }
}

async function serverRemove() {
  if (await confirm("Confirmed to remove the server?") && vServerSelect.value) {
    let index = vServerSelect.value.selectedIndex;
    vServers.value.splice(index, 1);
    await nextTick();
    vServerSelect.value.selectedIndex = Math.max(0, index - 1);
    onSelectedServerChanged();
  }
}

function onSelectedServerChanged() {
  let indexId = vServerSelect.value?.selectedOptions[0].value;
  if (indexId == null) return;
  let vmessItem = serverFind(indexId)
  if (vmessItem == null) return;
  vServerRemarks.value = vmessItem.remarks;
  vServerAddress.value = vmessItem.address;
  vServerPort.value = vmessItem.port;
  vServerId.value = vmessItem.id;
  vServerFlow.value = vmessItem.flow;
  vServerPath.value = vmessItem.path;
  vServerNetwork.value = vmessItem.network;
  vServerStreamSecurity.value = vmessItem.streamSecurity;
  vServerSni.value = vmessItem.sni;
  vServerFingerPrint.value = vmessItem.fingerprint;
  vServerPublicKey.value = vmessItem.publicKey;
  vServerShortId.value = vmessItem.shortId;
  vServerSpiderX.value = vmessItem.spiderX;
  vServerAlpn.value = vmessItem.alpn[0];
  vServerAllowInsecure.value = vmessItem.allowInsecure;
}

async function applyServerConfig() {
  if (vServerSelect.value && vServerSelect.value.selectedIndex >= 0) {
    let vmessItem = vServers.value[vServerSelect.value.selectedIndex];
    vmessItem.remarks = vServerRemarks.value;
    vmessItem.address = vServerAddress.value;
    vmessItem.port = vServerPort.value;
    vmessItem.id = vServerId.value;
    vmessItem.flow = vServerFlow.value;
    vmessItem.path = vServerPath.value;
    vmessItem.network = vServerNetwork.value;
    vmessItem.streamSecurity = vServerStreamSecurity.value;
    vmessItem.sni = vServerSni.value;
    vmessItem.fingerprint = vServerFingerPrint.value;
    vmessItem.publicKey = vServerPublicKey.value;
    vmessItem.shortId = vServerShortId.value;
    vmessItem.spiderX = vServerSpiderX.value;
    vmessItem.alpn = [vServerAlpn.value];
    vmessItem.allowInsecure = vServerAllowInsecure.value;
    await writeTextFile(vAppConfigPath, JSON.stringify(vAppConfig));
  }
}

async function applyAppConfig() {
  vAppConfig.inbound[0].localPort = Math.floor(vAppConfigPort.value);
  await writeTextFile(vAppConfigPath, JSON.stringify(vAppConfig));
}

function serverSelect() {
  if (vServerSelect.value != null) {
    let index = vServers.value.findIndex((element) => element.indexId == vAppConfig.indexId);
    vServerSelect.value.selectedIndex = index;
  }
  onSelectedServerChanged();
}

function serverFind(indexId: string) {
  if (vServerSelect.value != null) {
    return vServers.value.find((element) => element.indexId == indexId);
  }
}

async function appExit() {
  if (await confirm("Exit?")) {
    await exit()
  }
}

onMounted(async () => {
  const configDirectory = (await configDir()) + "vtauray/";
  vAppConfigPath = configDirectory + "guiNConfig.json";
  vConfigPath = configDirectory + "config.json";

  let vConfigContent = await readTextFile(vAppConfigPath);
  vAppConfig = JSON.parse(vConfigContent);

  vServers.value = vAppConfig.vmess;
  vAppConfigPort.value = vAppConfig.inbound[0].localPort;

  listen('v-connect', (_: Event<number>) => v2rayConnect());
  listen('v-disconnect', (_: Event<number>) => v2rayDisconnect());

  await nextTick();
  serverSelect();
});
</script>

<template>
  <div class="grid-container">
    <div class="div-helm">
      <Helm @e-exit="appExit" @e-rmv-server="serverRemove" @e-add-server="serverAdd" @e-disconnect="v2rayDisconnect"
        @e-connect="v2rayConnect" />
    </div>

    <div class="div-server-list">
      <select class="servers" ref="vServerSelect" @change="onSelectedServerChanged" id="v-server-select"
        contenteditable="false" size="2">
        <option v-for="item in vServers" :key="item.indexId" :value="item.indexId">
          {{ item.remarks }}
        </option>
      </select>
    </div>

    <div class="div-dashboard">
      <div class="tabs">
        <input class="input" name="tabs" type="radio" id="tab-1" checked />
        <label class="label" for="tab-1">vtauray</label>
        <div class="panel">
          <!-- <img src="./assets/vtauray512.svg"> -->
          <button id="x-update" style="width: 100px;" @click="xUpdate">Update</button>
        </div>
        <input class="input" name="tabs" type="radio" id="tab-2" />
        <label class="label" for="tab-2">server</label>
        <div class="panel">
          <div class="settings">
            <label>别名(remarks)</label>
            <input type="text" id="v-server-remarks" v-model="vServerRemarks" />
            <label>地址(address)</label>
            <input type="text" id="v-server-address" v-model="vServerAddress" />
            <label>端口(port)</label>
            <input type="text" id="v-server-port" v-model="vServerPort" />
            <label>用户ID(id)</label>
            <input type="text" id="v-server-id" v-model="vServerId" />
            <label>流控(flow)</label>
            <select id="v-server-flow" v-model="vServerFlow">
              <option disabled value="">Please Select one</option>
              <option value="xtls-rprx-vision">xtls-rprx-vision</option>
              <option value="xtls-rprx-vision-udp443">xtls-rprx-vision-udp443</option>
            </select>
            <label>传输协议(network)</label>
            <select id="v-server-network" v-model="vServerNetwork">
              <option disabled value="">Please Select one</option>
              <option value="tcp">tcp</option>
              <option value="ws">ws</option>
            </select>
            <label>路径(path)</label>
            <input type="text" id="v-server-path" v-model="vServerPath" />
            <label>传输层安全(tls)</label>
            <select id="v-server-streamSecurity" v-model="vServerStreamSecurity">
              <option disabled value="">Please Select one</option>
              <option value="tls">tls</option>
              <option value="reality">reality</option>
            </select>
            <label>SNI</label>
            <input type="text" id="v-server-sni" v-model="vServerSni" />
            <label>Fingerprint</label>
            <select id="v-server-fingerprint" v-model="vServerFingerPrint">
              <option disabled value="">Please Select one</option>
              <option value="chrome">chrome</option>
              <option value="firefox">firefox</option>
              <option value="edge">edge</option>
            </select>
            <label v-if="vServerStreamSecurity === 'reality'">PublicKey</label>
            <input v-if="vServerStreamSecurity === 'reality'" type="text" id="v-server-publicKey"
              v-model="vServerPublicKey" />
            <label v-if="vServerStreamSecurity === 'reality'">ShortId</label>
            <input v-if="vServerStreamSecurity === 'reality'" type="text" id="v-server-shortId"
              v-model="vServerShortId" />
            <label v-if="vServerStreamSecurity === 'reality'">SpiderX</label>
            <input v-if="vServerStreamSecurity === 'reality'" type="text" id="v-server-spiderX"
              v-model="vServerSpiderX" />
            <label v-if="vServerStreamSecurity === 'tls'">alpn</label>
            <select v-if="vServerStreamSecurity === 'tls'" id="v-server-alpn" v-model="vServerAlpn">
              <option disabled value="">Please Select one</option>
              <option value="http/1.1">http/1.1</option>
              <option value="h2">h2</option>
            </select>
            <label v-if="vServerStreamSecurity === 'tls'">跳过证书验证</label>
            <select v-if="vServerStreamSecurity === 'tls'" id="v-server-allowInsecure" v-model="vServerAllowInsecure">
              <option disabled value="">Please Select one</option>
              <option value="true">true</option>
              <option value="false">false</option>
            </select>
          </div>
          <button id="v-server-apply" @click="applyServerConfig">apply</button>
        </div>
        <input class="input" name="tabs" type="radio" id="tab-3" />
        <label class="label" for="tab-3">settings</label>
        <div class="panel">
          <div class="settings">
            <label>本地监听端口</label>
            <input type="text" id="v-app-config-port" v-model="vAppConfigPort" />
          </div>
          <button id="v-config-apply" @click="applyAppConfig">apply</button>
        </div>
      </div>
    </div>

    <div class="div-logging">
      <Log />
    </div>

    <div class="div-status-line">
      <StatusLine />
    </div>
  </div>
</template>

<style scoped>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.grid-container {
  display: grid;
  grid-template-columns: 255px auto;
  grid-template-rows: auto 110px 20px 40px;
  gap: 5px;
  width: 100vw;
  height: 100vh;
}

.div-helm {
  grid-column: 1 / 2;
  grid-row: 3 / 4;
  align-items: stretch;
}

.div-server-list {
  grid-column: 1 / 2;
  grid-row: 1 / 3;
}

.div-dashboard {
  grid-column: 2 / 3;
  grid-row: 1 / 2;
  border: lightgray;
  border-width: 1px;
  border-radius: 5px;
  border-style: solid;
}

.div-logging {
  grid-column: 2 / 3;
  grid-row: 2 / 4;
  border: lightgray;
  border-width: 1px;
  border-radius: 5px;
  border-style: solid;
}

.div-status-line {
  grid-column: 1 / 3;
  grid-row: 4 / 5;
  background-color: #e5e5e5;
}

.servers {
  display: flex;
  flex-wrap: wrap;
  width: 100%;
  height: 100%;
  overflow: scroll;
  resize: none;
  background: white;
  border-width: 1px;
  outline-style: unset;
}

.server-input {
  position: absolute;
  opacity: 0;
}

.server-label {
  width: 100%;
  height: 60px;
  background: #e5e5e5;
  cursor: pointer;
  font-size: 13px;
  color: #7f7f7f;
  transition: background 0.1s, color 0.1s;
}

.tabs {
  display: flex;
  flex-wrap: wrap;
  background: #e5e5e5;
}

.input {
  position: absolute;
  opacity: 0;
}

.label {
  width: 100%;
  padding: 8px 30px 16px 30px;
  height: 8px;
  background: #e5e5e5;
  cursor: pointer;
  font-size: 13px;
  color: #7f7f7f;
  transition: background 0.1s, color 0.1s;
}

.label:hover {
  background: #d8d8d8;
}

.label:active {
  background: #ccc;
}

.input:focus+.label {
  z-index: 1;
}

.input:checked+.label {
  background: #fff;
  color: #000;
}

@media (min-width: 600px) {
  .label {
    width: auto;
  }
}

.panel {
  display: none;
  overflow: scroll;
  height: calc(100vh - 245px);
  width: 100%;
  padding: 10px 30px 0px;
  background: #fff;
}

.panel label {
  font-size: 13px;
}

.panel input {
  font-size: 13px;
}

.panel select {
  font-size: 13px;
}

div.settings {
  display: grid;
  grid-template-columns: max-content max-content;
  grid-gap: 5px;
}

div.settings label {
  text-align: left;
}

div.settings label:after {
  content: ":";
}

@media (min-width: 600px) {
  .panel {
    order: 99;
  }
}

.input:checked+.label+.panel {
  display: block;
}</style>
