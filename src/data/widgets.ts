import type { Widget } from "../types";
import { invoke } from '@tauri-apps/api/core';

export const getWidgets = async (): Promise<Widget[]> => {
  return invoke<Widget[]>(
    "sync_and_get_manifest",
  );
};