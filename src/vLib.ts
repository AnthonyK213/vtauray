import { invoke } from "@tauri-apps/api/tauri";

export async function vConnect() {
  await invoke("v2ray_connect");
}

export async function vDisconnect() {
  await invoke("v2ray_disconnect");
}

export async function vReadFile(path: string) {
  return await invoke("read_file", { "path": path }).catch((error) => console.error(error));
}

export async function vWriteFile(path: string, content: string) {
  return await invoke("write_file", { "path": path, "content": content }).catch((error) => console.error(error));
}
