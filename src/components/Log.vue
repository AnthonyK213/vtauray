<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { LogPayload } from '../interface';
import { Event, listen } from '@tauri-apps/api/event';

const vLogging = ref<HTMLDivElement | null>(null);

onMounted(async () => {
  listen('v-logging', (event: Event<LogPayload>) => {
    let colors = ["black", "orange", "red"];
    const { m_type, message } = event.payload;
    let log = `<p style="color:${colors[m_type]};">${message}</p>`;
    if (vLogging.value == null) return;
    if (vLogging.value.innerHTML.length >= 65001) {
      vLogging.value.innerHTML = log;
    } else {
      vLogging.value.innerHTML += log;
    }
    vLogging.value.scrollTop = vLogging.value.scrollHeight;
  });
})
</script>

<template>
  <div class="logging" id="v-logging" ref="vLogging" contenteditable="false">
    <div style="color: orange;">
      Hello, vtauray!
    </div>
  </div>
</template>

<style scoped>
.logging {
  font-family: monospace;
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
  border-radius: 5px;
  border-color: lightgray;
}
</style>
