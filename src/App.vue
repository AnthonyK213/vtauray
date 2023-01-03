<script setup lang="ts">
import * as vLib from './vLib';
import { emit, Event, listen } from '@tauri-apps/api/event';
import { confirm } from '@tauri-apps/api/dialog';
import { reactive, ref, onMounted } from 'vue';

const vLogging = ref<HTMLDivElement | null>(null);

interface Payload {
  m_type: number,
  message: string,
}

async function v2rayConnect() {
  await vLib.vConnect();
}

async function v2rayDisconnect() {
  await vLib.vDisconnect();
}

onMounted(() => {
  const unlisten_logging = listen('v-logging', (event: Event<Payload>) => {
    let colors = ["black", "orange", "red"];
    const { m_type, message } = event.payload;
    let log = `<p style="color:${colors[m_type]};">${message}</p>`;
    if (vLogging.value === null) return;
    if (vLogging.value.innerHTML.length >= 65001) {
      vLogging.value.innerHTML = log;
    } else {
      vLogging.value.innerHTML += log;
    }
    vLogging.value.scrollTop = vLogging.value.scrollHeight;
  });
});


// function vConfigRemoveByIndexId(indexId) {
//   vConfig.vmess = vConfig.vmess.filter((v, x, y) => {
//     return v.indexId !== indexId;
//   });
// }
// 
// function vServerAdd(remarks, indexId, fire = true) {
//   if (indexId === '0') {
//     indexId = Math.round(new Date()) + "";
//   }
//   let newOption = new Option(remarks, indexId);
//   vServers.add(newOption, undefined);
//   vServers.selectedIndex = vServers.options.length - 1;
//   // Add to vConfig.vmess.
//   if (fire) {
//     vConfig.vmess.push({
//       "indexId": indexId,
//       "address": "",
//       "port": 443,
//       "id": "",
//       "alterId": 0,
//       "security": "none",
//       "network": "tcp",
//       "remarks": remarks,
//       "headerType": "none",
//       "requestHost": "",
//       "path": "",
//       "streamSecurity": "tls",
//       "allowInsecure": "true",
//       "testResult": "  4.2 M/s",
//       "subid": "",
//       "flow": "",
//       "sni": "",
//       "alpn": [
//         "http/1.1"
//       ],
//       "groupId": "",
//       "coreType": 2,
//       "preSocksPort": 0
//     });
//     onServerSelectionChanged();
//   }
// }
// 
// async function vServerRemove(fire = true) {
//   if (await confirm("Really?")) {
//     let index = vServers.selectedIndex;
//     if (index >= 0) {
//       vConfigRemoveByIndexId(vServers.options[index].value);
//       vServers.remove(index);
//       if (index >= 1) {
//         vServers.selectedIndex = index - 1;
//         if (fire) {
//           onServerSelectionChanged();
//         }
//       }
//       return item;
//     }
//   }
// }
// 
// function getSelectedServer() {
//   if (vServers.options.length === 0) return;
//   let indexId = vServers.options[vServers.selectedIndex].value;
//   for (const item of vConfig.vmess) {
//     if (item.indexId === indexId) {
//       return item;
//     }
//   }
// }
// 
// function setSelectIndex(select, val) {
//   for (let i = 0; i < select.options.length; i++) {
//     if (select.options[i].value === val) {
//       select.selectedIndex = i;
//       break;
//     }
//   }
// }
// 
// function vServerUpdate() {
//   let selectedServer = getSelectedServer();
// 
//   if (!selectedServer) {
//     return;
//   }
// 
//   if (vServerRemarks.value !== selectedServer.remarks) {
//     selectedServer.remarks = vServerRemarks.value;
//     vServers.options[vServers.selectedIndex].text = vServerRemarks.value;
//   }
//   selectedServer.address = vServerAddress.value;
//   selectedServer.port = parseInt(vServerPort.value);
//   selectedServer.id = vServerId.value;
//   selectedServer.path = vServerPath.value;
//   selectedServer.network = vServerNetwork.options[vServerNetwork.selectedIndex].value;
//   selectedServer.streamSecurity = vServerStreamSecurity.options[vServerStreamSecurity.selectedIndex].value;
//   selectedServer.allowInsecure = vServerAllowInsecure.options[vServerAllowInsecure.selectedIndex].value;
//   selectedServer.alpn = [vServerAlpn.options[vServerAlpn.selectedIndex].value];
// 
//   //console.log(JSON.stringify(selectedServer));
//   vWriteFile("app_config.json", JSON.stringify(vConfig));
// }
// 
// function vServerUndo() { }
// 
// function onServerSelectionChanged() {
//   if (vServers.options.length === 0) return;
//   let selection = vServers.options[vServers.selectedIndex];
//   if (true || selection.text !== 'Unnamed') {
//     for (const item of vConfig.vmess) {
//       if (item.indexId === selection.value) {
//         vServerRemarks.value = item.remarks;
//         vServerAddress.value = item.address;
//         vServerPort.value = item.port;
//         vServerId.value = item.id;
//         vServerPath.value = item.path;
//         setSelectIndex(vServerNetwork, item.network);
//         setSelectIndex(vServerStreamSecurity, item.streamSecurity);
//         setSelectIndex(vServerAllowInsecure, item.allowInsecure);
//         setSelectIndex(vServerAlpn, item.alpn[0]);
//         break;
//       }
//     }
//   }
// }
// 
// function generateV2rayConfig() {
// 
//   let server = getSelectedServer();
// 
//   if (!server) return;
// 
//   let config = {
//     "policy": {
//       "system": {
//         "statsOutboundUplink": true,
//         "statsOutboundDownlink": true
//       }
//     },
//     "log": {
//       "access": "",
//       "error": "",
//       "loglevel": "warning"
//     },
//     "inbounds": [
//       {
//         "tag": "socks",
//         "port": vConfig.inbound[0].localPort,
//         "listen": "127.0.0.1",
//         "protocol": "socks",
//         "sniffing": {
//           "enabled": true,
//           "destOverride": [
//             "http",
//             "tls"
//           ]
//         },
//         "settings": {
//           "auth": "noauth",
//           "udp": true,
//           "allowTransparent": false
//         }
//       },
//       {
//         "tag": "http",
//         "port": vConfig.inbound[0].localPort + 1,
//         "listen": "127.0.0.1",
//         "protocol": "http",
//         "sniffing": {
//           "enabled": true,
//           "destOverride": [
//             "http",
//             "tls"
//           ]
//         },
//         "settings": {
//           "auth": "noauth",
//           "udp": true,
//           "allowTransparent": false
//         }
//       },
//       {
//         "tag": "socks2",
//         "port": 10810,
//         "listen": "0.0.0.0",
//         "protocol": "socks",
//         "sniffing": {
//           "enabled": true,
//           "destOverride": [
//             "http",
//             "tls"
//           ]
//         },
//         "settings": {
//           "auth": "noauth",
//           "udp": true,
//           "allowTransparent": false
//         }
//       },
//       {
//         "tag": "http2",
//         "port": 10811,
//         "listen": "0.0.0.0",
//         "protocol": "http",
//         "sniffing": {
//           "enabled": true,
//           "destOverride": [
//             "http",
//             "tls"
//           ]
//         },
//         "settings": {
//           "auth": "noauth",
//           "udp": true,
//           "allowTransparent": false
//         }
//       },
//       {
//         "tag": "api",
//         "port": 49723,
//         "listen": "127.0.0.1",
//         "protocol": "dokodemo-door",
//         "settings": {
//           "udp": false,
//           "address": "127.0.0.1",
//           "allowTransparent": false
//         }
//       }
//     ],
//     "outbounds": [
//       {
//         "tag": "proxy",
//         "protocol": "vless",
//         "settings": {
//           "vnext": [
//             {
//               "address": server.address,
//               "port": server.port,
//               "users": [
//                 {
//                   "id": server.id,
//                   "alterId": server.alterId,
//                   "email": "t@t.tt",
//                   "security": "auto",
//                   "encryption": "none",
//                   "flow": ""
//                 }
//               ]
//             }
//           ]
//         },
//         "streamSettings": {
//           "network": server.network,
//           "security": server.streamSecurity,
//           "tlsSettings": {
//             "allowInsecure": server.allowInsecure === "true",
//             "alpn": server.alpn
//           },
//           "wsSettings": {
//             "path": server.path
//           }
//         },
//         "mux": {
//           "enabled": false,
//           "concurrency": -1
//         }
//       },
//       {
//         "tag": "direct",
//         "protocol": "freedom",
//         "settings": {}
//       },
//       {
//         "tag": "block",
//         "protocol": "blackhole",
//         "settings": {
//           "response": {
//             "type": "http"
//           }
//         }
//       }
//     ],
//     "stats": {},
//     "api": {
//       "tag": "api",
//       "services": [
//         "StatsService"
//       ]
//     },
//     "routing": {
//       "domainStrategy": "IPIfNonMatch",
//       "domainMatcher": "linear",
//       "rules": [
//         {
//           "type": "field",
//           "inboundTag": [
//             "api"
//           ],
//           "outboundTag": "api",
//           "enabled": true
//         },
//         {
//           "type": "field",
//           "inboundTag": [],
//           "outboundTag": "direct",
//           "domain": [
//             "domain:example-example.com",
//             "domain:example-example2.com"
//           ],
//           "enabled": true
//         },
//         {
//           "type": "field",
//           "inboundTag": [],
//           "outboundTag": "block",
//           "domain": [
//             "geosite:category-ads-all"
//           ],
//           "enabled": true
//         },
//         {
//           "type": "field",
//           "outboundTag": "direct",
//           "domain": [
//             "geosite:cn"
//           ],
//           "enabled": true
//         },
//         {
//           "type": "field",
//           "outboundTag": "direct",
//           "ip": [
//             "geoip:private",
//             "geoip:cn"
//           ],
//           "enabled": true
//         },
//         {
//           "type": "field",
//           "port": "0-65535",
//           "outboundTag": "proxy",
//           "enabled": true
//         }
//       ]
//     }
//   };
// 
//   return config;
// }
// 
// 
// // document.addEventListener('contextmenu', (e) => e.preventDefault());
// 
// window.addEventListener("DOMContentLoaded", async () => {
//   vStdout = document.querySelector("#v-logging");
//   vServers = document.querySelector("#v-servers");
//   vServerRemarks = document.querySelector("#v-server-remarks");
//   vServerAddress = document.querySelector("#v-server-address");
//   vServerPort = document.querySelector("#v-server-port");
//   vServerId = document.querySelector("#v-server-id");
//   vServerPath = document.querySelector("#v-server-path");
//   vServerNetwork = document.querySelector("#v-server-network");
//   vServerStreamSecurity = document.querySelector("#v-server-streamSecurity");
//   vServerAllowInsecure = document.querySelector("#v-server-allowInsecure");
//   vServerAlpn = document.querySelector("#v-server-alpn");
// 
//   let configString = await vReadFile("app_config.json");
//   vConfig = JSON.parse(configString);
//   if (vConfig.vmess) {
//     for (const item of vConfig.vmess) {
//       vServerAdd(item.remarks, item.indexId, false);
//     }
//   }
// 
//   setSelectIndex(vServers, vConfig.indexId);
//   onServerSelectionChanged();
// 
//   document.querySelector("#v-config-port").value = vConfig.inbound[0].localPort;
//   //const vConfigPortocol = document.querySelector("#v-config-protocol");
//   //for (let protocolIndex = 0; protocolIndex < vConfigPortocol.options.length; protocolIndex++) {
//   //if (vConfigPortocol.options[protocolIndex].value === vConfig.inbound[0].protocol) {
//   //vConfigPortocol.selectedIndex = protocolIndex;
//   //break;
//   //}
//   //}
// 
//   document.querySelector("#v-add-server")?.addEventListener("click", () => vServerAdd('Unnamed', '0'));
//   // document.querySelector("#v-rmv-server")?.addEventListener("click", vServerRemove);
//   document.querySelector("#v-servers")?.addEventListener("change", onServerSelectionChanged);
//   document.querySelector("#v-server-apply")?.addEventListener("click", vServerUpdate);
// });
</script>

<template>
  <div class="grid-container">
    <div class="box0">
      <!-- <button id="v-undo" style="height: 20px;width: 20px;">&lt;</button>
      <button id="v-rmv-server" style="height: 20px;width: 20px;">-</button>
      <button id="v-add-server" style="height: 20px;width: 20px;">+</button> -->
      <button id="v-disconnect" @click="v2rayDisconnect" style="height: 20px;width: 80px;">disconnect</button>
      <button id="v-connect" @click="v2rayConnect" style="height: 20px;width: 80px;">connect</button>
    </div>

    <div class="box1">
      <select class="servers" id="v-servers" contenteditable="false" multiple>
      </select>
    </div>

    <div class="box2">
      <div class="tabs">
        <input class="input" name="tabs" type="radio" id="tab-1" checked />
        <label class="label" for="tab-1">vtauray</label>
        <div class="panel">
          <center>
            <img src="./assets/vue.svg">
          </center>
        </div>
        <input class="input" name="tabs" type="radio" id="tab-2" />
        <label class="label" for="tab-2">server</label>
        <div class="panel">
          <div class="settings">
            <label>别名(remarks)</label>
            <input type="text" id="v-server-remarks" />
            <label>地址(address)</label>
            <input type="text" id="v-server-address" />
            <label>端口(port)</label>
            <input type="text" id="v-server-port" />
            <label>用户ID(id)</label>
            <input type="text" id="v-server-id" />
            <label>传输协议(network)</label>
            <select id="v-server-network">
              <option value="tcp">tcp</option>
              <option value="ws">ws</option>
            </select>
            <label>路径(path)</label>
            <input type="text" id="v-server-path" />
            <label>传输层安全(tls)</label>
            <select id="v-server-streamSecurity">
              <option value="tls">tls</option>
              <option value="xtls">xtls</option>
            </select>
            <label>跳过证书验证(allowInsecure)</label>
            <select id="v-server-allowInsecure">
              <option value="true">true</option>
              <option value="false">false</option>
            </select>
            <label>alpn</label>
            <select id="v-server-alpn">
              <option value="http/1.1">http/1.1</option>
              <option value="h2">h2</option>
            </select>
          </div>
          <button id="v-server-apply" style="height: 20px;">apply</button>
        </div>
        <input class="input" name="tabs" type="radio" id="tab-3" />
        <label class="label" for="tab-3">settings</label>
        <div class="panel">
          <div class="settings">
            <label>本地监听端口</label>
            <input type="text" id="v-config-port" />
          </div>
          <button id="v-config-apply" style="height: 20px;">apply</button>
        </div>
      </div>
    </div>

    <div class="box3">
      <div class="logging" id="v-logging" ref="vLogging" contenteditable="false">
        <p style="color: orange;">
          Hello, vtauray!
        </p>
      </div>
    </div>

    <div class="box4">
      <p style="color: black;">
        Status Bar
      </p>
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
  grid-template-columns: 260px 510px;
  grid-auto-rows: 380px 110px 20px 40px;
  gap: 10px;
}

.box0 {
  grid-column: 1 / 2;
  grid-row: 3 / 4;
}

.box1 {
  grid-column: 1 / 2;
  grid-row: 1 / 3;
}

.box2 {
  grid-column: 2 / 3;
  grid-row: 1 / 2;
}

.box3 {
  grid-column: 2 / 3;
  grid-row: 2 / 4;
  height: 100%;
}

.box4 {
  grid-column: 1 / 3;
  grid-row: 4 / 5;
  background-color: #e5e5e5;
}

.logging {
  font-family: 'Courier New', Courier, monospace;
  height: 100%;
  width: 100%;
  font-size: 13px;
  overflow: scroll;
  overflow-wrap: normal;
  overflow-x: scroll;
  resize: none;
  background-color: white;
  color: black;
  margin-left: auto;
  margin-right: auto;
  display: block;
  border-radius: 0px;
  border-color: lightgray;
}

.servers {
  display: flex;
  flex-wrap: wrap;
  width: 100%;
  height: 100%;
  overflow: scroll;
  resize: none;
  background: white;
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
  max-width: 700px;
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
  height: 295px;
  width: 100%;
  padding: 20px 30px 30px;
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
}
</style>
