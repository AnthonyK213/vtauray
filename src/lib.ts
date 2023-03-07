import { invoke } from "@tauri-apps/api/tauri";

export async function vConnect(vConfigPath: string) {
  await invoke("v2ray_connect", { path: vConfigPath }).catch((e) => console.log(e));
}

export async function vDisconnect() {
  await invoke("v2ray_disconnect");
}

export async function xUpdate() {
  await invoke("xray_update");
}