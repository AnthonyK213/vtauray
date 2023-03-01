<script setup lang="ts">
defineEmits<{
  (e: 'eUndo'): void,
  (e: 'eRmvServer'): void,
  (e: 'eAddServer'): void,
  (e: 'eDisconnect'): void,
  (e: 'eConnect'): void,
}>()
</script>

<template>
  <button id="v-undo" style="width: 20px;" @click="$emit('eUndo')">&lt;</button>
  <button id="v-rmv-server" style="width: 20px;" @click="$emit('eRmvServer')">-</button>
  <button id="v-add-server" style="width: 20px;" @click="$emit('eAddServer')">+</button>
  <button id="v-disconnect" @click="$emit('eDisconnect')" style="width: 85px;">disconnect</button>
  <button id="v-connect" @click="$emit('eConnect')" style="width: 80px;">connect</button>
</template>

<style>
button {
  display: block;
  margin-left: 3px;
  margin-right: 3px;
  font-size: 13px;
  align-content: center;
  float: left;
  height: 20px;
}
</style>
