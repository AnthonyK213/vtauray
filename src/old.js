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
//   document.querySelector("#v-rmv-server")?.addEventListener("click", vServerRemove);
//   document.querySelector("#v-servers")?.addEventListener("change", onServerSelectionChanged);
//   document.querySelector("#v-server-apply")?.addEventListener("click", vServerUpdate);
// });