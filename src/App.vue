<script setup lang="ts">
import { emit, Event, listen } from '@tauri-apps/api/event';
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
import { configDir } from '@tauri-apps/api/path';
import { confirm } from '@tauri-apps/api/dialog';
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

let vAppConfigPath: string
let vConfigPath: string;
let vAppConfig: AppConfig;
const vLogging = ref<HTMLDivElement | null>(null);
const vServers = ref<VmessItem[]>([]);
const vServerSelect = ref<HTMLSelectElement | null>(null);

const vServerRemarks = ref('');
const vServerAddress = ref('');
const vServerPort = ref(443);
const vServerId = ref('');
const vServerNetwork = ref('');
const vServerPath = ref('');
const vServerStreamSecurity = ref('');
const vServerAllowInsecure = ref('');
const vServerAlpn = ref('');

interface Payload {
  m_type: number,
  message: string,
}

interface InboundItem {
  localPort: number,
  protocol: string,
}

interface VmessItem {
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
  alpn: string[],
  groupId: string,
  coreType: number,
  preSocksPort: number,
}

interface AppConfig {
  indexId: string,
  inbound: InboundItem[],
  vmess: VmessItem[],
}

/* backend */

async function vConnect() {
  await invoke("v2ray_connect");
}

async function vDisconnect() {
  await invoke("v2ray_disconnect");
}

/* frontend */

async function v2rayConnect() {
  await vConnect();
}

async function v2rayDisconnect() {
  await vDisconnect();
}

function serverAdd() {

}

async function serverRemove() {
  if (await confirm("Really?")) {

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
}

async function applyServerConfig() {

}

function applyAppConfig() {

}

function serverSelect() {
  if (vServerSelect.value != null) {
    let index = vServers.value.findIndex((element) => element.indexId == vAppConfig.indexId);
    vServerSelect.value.selectedIndex = index;
    console.log("findIndex", vServerSelect.value.selectedIndex);
  }
}

function serverFind(indexId: string) {
  if (vServerSelect.value != null) {
    return vServers.value.find((element) => element.indexId == indexId);
  }
}

onMounted(async () => {
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

  const configDirectory = (await configDir()) + "vtauray/";
  vAppConfigPath = configDirectory + "app_config.json";
  vConfigPath = configDirectory + "config.json";

  let vConfigContent = await readTextFile(vAppConfigPath);
  vAppConfig = JSON.parse(vConfigContent);

  vServers.value = vAppConfig.vmess;
  serverSelect();
});
</script>

<template>
  <div class="grid-container">
    <div class="box0">
      <button id="v-undo" style="width: 20px;">&lt;</button>
      <button id="v-rmv-server" style="width: 20px;" @click="serverRemove">-</button>
      <button id="v-add-server" style="width: 20px;" @click="serverAdd">+</button>
      <button id="v-disconnect" @click="v2rayDisconnect" style="width: 85px;">disconnect</button>
      <button id="v-connect" @click="v2rayConnect" style="width: 80px;">connect</button>
    </div>

    <div class="box1">
      <select class="servers" ref="vServerSelect" @change="onSelectedServerChanged" id="v-server-select"
        contenteditable="false" multiple>
        <option v-for="item in vServers" :key="item.indexId" :value="item.indexId">
          {{ item.remarks }}
        </option>
      </select>
    </div>

    <div class="box2">
      <div class="tabs">
        <input class="input" name="tabs" type="radio" id="tab-1" checked />
        <label class="label" for="tab-1">vtauray</label>
        <div class="panel">
          <img src="./assets/vue.svg">
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
              <option value="xtls">xtls</option>
            </select>
            <label>跳过证书验证(allowInsecure)</label>
            <select id="v-server-allowInsecure" v-model="vServerAllowInsecure">
              <option disabled value="">Please Select one</option>
              <option value="true">true</option>
              <option value="false">false</option>
            </select>
            <label>alpn</label>
            <select id="v-server-alpn" v-model="vServerAlpn">
              <option disabled value="">Please Select one</option>
              <option value="http/1.1">http/1.1</option>
              <option value="h2">h2</option>
            </select>
          </div>
          <button id="v-server-apply" @click="applyServerConfig">apply</button>
        </div>
        <input class="input" name="tabs" type="radio" id="tab-3" />
        <label class="label" for="tab-3">settings</label>
        <div class="panel">
          <div class="settings">
            <label>本地监听端口</label>
            <input type="text" id="v-config-port" />
          </div>
          <button id="v-config-apply" @click="applyAppConfig">apply</button>
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
  grid-template-columns: 255px auto;
  grid-template-rows: auto 110px 20px 40px;
  gap: 5px;
  width: 100vw;
  height: 100vh;
}

.box0 {
  grid-column: 1 / 2;
  grid-row: 3 / 4;
  align-items: stretch;
}

.box1 {
  grid-column: 1 / 2;
  grid-row: 1 / 3;
}

.box2 {
  grid-column: 2 / 3;
  grid-row: 1 / 2;
  border: lightgray;
  border-width: 1px;
  border-radius: 5px;
  border-style: solid;
}

.box3 {
  grid-column: 2 / 3;
  grid-row: 2 / 4;
  border: lightgray;
  border-width: 1px;
  border-radius: 5px;
  border-style: solid;
}

.box4 {
  grid-column: 1 / 3;
  grid-row: 4 / 5;
  background-color: #e5e5e5;
}

button {
  display: block;
  margin-left: 3px;
  margin-right: 3px;
  font-size: 13px;
  align-content: center;
  float: left;
  height: 20px;
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
}
</style>
