<script setup lang="ts">
import { Event, listen } from '@tauri-apps/api/event';
import { onMounted, ref } from "vue";
import { StatsPayload } from '../interface';

const vSpeed = ref('0.0 B/s')

onMounted(async () => {
    listen('v-stats', (event: Event<StatsPayload>) => {
        const { outbound_proxy_traffic_downlink_speed } = event.payload;
        vSpeed.value = outbound_proxy_traffic_downlink_speed;
    });
})
</script>

<template>
    <p style="color: black;">
        {{ vSpeed }}
    </p>
</template>

<style></style>
