import { invoke } from '@tauri-apps/api/core'

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:native-titlebar|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}
